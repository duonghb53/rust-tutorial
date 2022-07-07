use derive_getters::Getters;
use derive_new::new;

use chrono::prelude::NaiveDate;

#[derive(Clone, Debug)]
pub enum MetricsTrend {
    DailyMetricsTrend(DailyMetricsTrend),
    WeeklyMetricsTrend(WeeklyMetricsTrend),
    MonthlyMetricsTrend(MonthlyMetricsTrend),
}

#[derive(Clone, Debug, Getters, new)]
pub struct DailyMetricsTrend {
    pub aggregation_dt: NaiveDate,
    pub value: f64,
}

#[derive(Clone, Debug, Getters, new)]
pub struct WeeklyMetricsTrend {
    pub start_dt: NaiveDate,
    pub end_dt: NaiveDate,
    pub value: f64,
}

#[derive(Clone, Debug, Getters, new)]
pub struct MonthlyMetricsTrend {
    pub start_dt: NaiveDate,
    pub end_dt: NaiveDate,
    pub value: f64,
}

fn convert_metric_trend_to_csv_row(
    convert_value_to_string: fn(f64) -> String,
) -> Box<dyn Fn(&MetricsTrend) -> Vec<String>> {
    Box::new(move |target_seller: &MetricsTrend| match target_seller {
        MetricsTrend::DailyMetricsTrend(trend) => vec![
            trend.aggregation_dt.format("%Y-%m-%d").to_string(),
            convert_value_to_string(trend.value),
        ],
        MetricsTrend::WeeklyMetricsTrend(trend) => {
            let label = format!(
                "{} - {}",
                trend.start_dt.format("%Y-%m-%d").to_string(),
                trend.end_dt.format("%Y-%m-%d").to_string(),
            );
            vec![label, convert_value_to_string(trend.value)]
        }
        MetricsTrend::MonthlyMetricsTrend(trend) => vec![
            trend.start_dt.format("%Y-%m").to_string(),
            convert_value_to_string(trend.value),
        ],
    })
}

fn transform_metrics_trend_results_to_csv_row(
    target_seller: Vec<MetricsTrend>,
    comparison_seller: Option<Vec<MetricsTrend>>,
    convert_value_to_string: fn(f64) -> String,
) -> Vec<Vec<String>> {
    let mut results = target_seller
        .iter()
        .map(convert_metric_trend_to_csv_row(convert_value_to_string))
        .collect::<Vec<_>>();

    if let Some(comparison_seller) = comparison_seller {
        let comparison_seller_results = comparison_seller
            .iter()
            .map(convert_metric_trend_to_csv_row(convert_value_to_string))
            .collect::<Vec<_>>();

        results = results
            .iter()
            .zip(comparison_seller_results.iter())
            .into_iter()
            .map(|(a, b)| vec![a[0].to_owned(), a[1].to_owned(), b[1].to_owned()])
            .collect::<Vec<_>>();
    }

    results
}

fn main() {
    let daily_metrics_trends: Vec<MetricsTrend> = vec![
        MetricsTrend::DailyMetricsTrend(DailyMetricsTrend {
            aggregation_dt: NaiveDate::from_ymd(2021, 5, 17),
            value: 9.45,
        }),
        MetricsTrend::DailyMetricsTrend(DailyMetricsTrend {
            aggregation_dt: NaiveDate::from_ymd(2021, 5, 18),
            value: 15.45,
        }),
    ];
    let daily_metrics_trends2: Vec<MetricsTrend> = vec![
        MetricsTrend::DailyMetricsTrend(DailyMetricsTrend {
            aggregation_dt: NaiveDate::from_ymd(2021, 5, 17),
            value: 12.45,
        }),
        MetricsTrend::DailyMetricsTrend(DailyMetricsTrend {
            aggregation_dt: NaiveDate::from_ymd(2021, 5, 18),
            value: 45.45,
        }),
    ];

    let daily_results = transform_metrics_trend_results_to_csv_row(
        daily_metrics_trends,
        Some(daily_metrics_trends2),
        |value: f64| format!("{:.2}", value),
    );

    println!("daily results: {:?}", daily_results);

    let weekly_metrics_trends: Vec<MetricsTrend> = vec![
        MetricsTrend::WeeklyMetricsTrend(WeeklyMetricsTrend {
            start_dt: NaiveDate::from_ymd(2021, 5, 17),
            end_dt: NaiveDate::from_ymd(2021, 5, 23),
            value: 9.45,
        }),
        MetricsTrend::WeeklyMetricsTrend(WeeklyMetricsTrend {
            start_dt: NaiveDate::from_ymd(2021, 5, 24),
            end_dt: NaiveDate::from_ymd(2021, 5, 31),
            value: 19.45,
        }),
    ];

    let weekly_results =
        transform_metrics_trend_results_to_csv_row(weekly_metrics_trends, None, |value: f64| {
            format!("{:.0}", value)
        });

    println!("weekly results: {:?}", weekly_results);

    let monthly_metrics_trends: Vec<MetricsTrend> = vec![
        MetricsTrend::MonthlyMetricsTrend(MonthlyMetricsTrend {
            start_dt: NaiveDate::from_ymd(2021, 5, 1),
            end_dt: NaiveDate::from_ymd(2021, 5, 31),
            value: 9.45,
        }),
        MetricsTrend::MonthlyMetricsTrend(MonthlyMetricsTrend {
            start_dt: NaiveDate::from_ymd(2021, 6, 1),
            end_dt: NaiveDate::from_ymd(2021, 6, 30),
            value: 19.45,
        }),
    ];

    let monthly_results =
        transform_metrics_trend_results_to_csv_row(monthly_metrics_trends, None, |value: f64| {
            format!("{:.4}", value)
        });

    println!("monthly results: {:?}", monthly_results);
}