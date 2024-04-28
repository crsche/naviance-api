#![feature(let_chains)]
#[macro_use]
extern crate tracing;

use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;
use futures::{stream, StreamExt, TryStreamExt};
use naviance::{types::*, util::sat_to_act, Client};
use reqwest::ClientBuilder;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Parser, Debug)]
struct Opts {
    #[clap(short, long, env)]
    /// if you don't know how to find this, you probably shouldn't be using it
    pub key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // Contains KEY
    let opts = Opts::parse();
    LogTracer::init().expect("Failed to initialize LogTracer");
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global tracing subscriber");

    // HTTP client
    let c = ClientBuilder::new()
        .use_rustls_tls()
        .https_only(true)
        .brotli(true)
        .gzip(true)
        .zstd(true)
        .deflate(true)
        .build()?;

    let client = Client::new_with_client(opts.key, c).await?;

    let schools = client.get_schools_im_thinking_about().await?;

    stream::iter(schools.data.into_iter())
        .map(|school| {
            let client = client.clone();
            tokio::spawn(async move {
                let name = school
                    .college
                    .as_ref()
                    .and_then(|c| c.name.as_ref())
                    .map(|s| s.as_str())
                    .unwrap_or("NO NAME");
                if let Some(uuid) = school.college.as_ref().and_then(|c| c.uuid) {
                    let stats = client.get_application_stats_by_uuid(&uuid).await?;
                    if let Some(scattergrams) = stats.scattergrams
                        && let Some(gpa) = scattergrams.gpa
                    {
                        // Convert SAT apps to ACT apps
                        let mut all = gpa
                            .sat
                            .as_ref()
                            .and_then(|sat| sat.apps.as_ref())
                            .map(|apps| apps.all().into_iter().cloned().collect::<Vec<_>>())
                            .unwrap_or_default();

                        let mut accepted = gpa
                            .sat
                            .as_ref()
                            .and_then(|sat| sat.apps.as_ref())
                            .map(|apps| apps.accepted().into_iter().cloned().collect::<Vec<_>>())
                            .unwrap_or_default();
                        // .sat
                        // .as_ref()
                        // .and_then(|sat| sat.apps.as_ref())
                        // .map(|apps| {
                        //     apps.accepted()
                        //         .into_iter()
                        //         .map(|a| a.to_act())
                        //         .collect::<Vec<_>>()
                        // })
                        // .unwrap_or_default();

                        // TODO: Don't clone
                        // if let Some(act_apps) = gpa.act.as_ref().and_then(|act|
                        // act.apps.as_ref()) {     all.extend(act_apps.
                        // all().into_iter().cloned());     accepted.
                        // extend(act_apps.accepted().into_iter().cloned());
                        // } else {
                        //     // warn!("No ACT data for school: {name}",);
                        // }

                        let mut type_map: HashMap<TypeName, (u32, u32)> = HashMap::new();
                        let mut boxed_type_map: HashMap<TypeName, (u32, u32)> = HashMap::new();
                        // let mut accepted_type_map = HashMap::new();

                        let (sat, gpa) = stats
                            .user_info
                            .and_then(|u: UserInfo| {
                                u.academics
                                    .map(|a| (a.sat.unwrap(), a.raw_cumulative_gpa.unwrap()))
                            })
                            .unwrap();
                        // let act = sat_to_act(sat);
                        // let act_range = act - 2..=act;
                        let sat_range = sat - 20..=sat + 30;
                        let gpa_range = gpa - 0.21..=gpa + 0.11;

                        for app in all.iter() {
                            let app_type = app.type_name.clone().unwrap_or(TypeName::Unknown);
                            let test = app.highest_combo_sat.unwrap();
                            let gpa = app.gpa.unwrap();
                            type_map.entry(app_type).or_default().1.add_assign(1);
                            if sat_range.contains(&test) && gpa_range.contains(&gpa) {
                                boxed_type_map.entry(app_type).or_default().1.add_assign(1);
                            }
                        }

                        for accepted in accepted.iter() {
                            let app_type = accepted.type_name.clone().unwrap_or(TypeName::Unknown);
                            let test = accepted.highest_combo_sat.unwrap();
                            let gpa = accepted.gpa.unwrap();
                            type_map.entry(app_type).or_default().0.add_assign(1);
                            if sat_range.contains(&test) && gpa_range.contains(&gpa) {
                                boxed_type_map
                                    .entry(app_type)
                                    .and_modify(|r| r.0.add_assign(1));
                            }
                        }

                        let accepts = accepted.len();
                        let total = all.len();
                        let total_rate = accepts as f64 * 100. / total as f64;

                        let accepts_boxed = boxed_type_map.values().map(|(a, _)| a).sum::<u32>();
                        let total_boxed = boxed_type_map.values().map(|(_, t)| t).sum::<u32>();
                        let boxed_rate = accepts_boxed as f64 * 100. / total_boxed as f64;

                        // println!("{type_map:?}");
                        // println!("{boxed_type_map:?}");
                        println!("{name}");
                        println!("\tTotal: {total} ({total_rate:.2}%)");
                        type_map.iter().for_each(|(k, (a, t))| {
                            let rate = *a as f64 * 100. / *t as f64;
                            println!("\t\t{k:?}: {a}/{t} ({rate:.2}%)",);
                        });
                        println!("\tBoxed: {total_boxed} ({boxed_rate:.2}%)");
                        // println!("{boxed_type_map:?}");
                        boxed_type_map.iter().for_each(|(k, (a, t))| {
                            let rate = *a as f64 * 100. / *t as f64;
                            println!("\t\t{k:?}: {a}/{t} ({rate:.2}%)",);
                        });
                        println!();
                    }
                } else {
                    warn!("No UUID for school: {name}");
                }
                Ok(())
            })
        })
        .buffer_unordered(16)
        .try_collect::<Vec<Result<_>>>()
        .await?
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    Ok(())
}
