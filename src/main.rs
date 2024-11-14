use chrono::prelude::*;
use gumdrop::Options;
use strum_macros::EnumString;

const MABV: [&str;13] = ["","J","F","M","A","Y","U","L","G","S","O","N","D"];

#[derive(Debug, Options)]
struct Cli {
    #[options(help = "Year variant, default is C20")]
    year: Option<EarthdateYearVariant>,
    #[options(help = "Month variant, default is Alpha")]
    month: Option<EarthdateMonthVariant>,
    #[options(help = "Time variant, default is Metric")]
    timev: Option<EarthdateTimeVariant>,

    #[options(help = "print help message")]
    help: bool,
    #[options(command)]
    command: Option<Command>, 
}   

#[derive(Debug, Options)]
enum Command {
    #[options(help = "Get just metric time")]
    Beat(BeatArgs),
}

#[derive(Debug, Options)]
struct BeatArgs {
    #[options(help = "Timezone to use, default is UTC")]
    timezone: Option<String>,
    #[options(help = "Precision of output, default is 2")]
    precision: Option<u8>,
}

fn main() {
    let time = Local::now();
    let cli = Cli::parse_args_default_or_exit();

    

    if cli.command.is_none() {
        println!("{}", earthdate(time, cli.year.unwrap_or(EarthdateYearVariant::C20), cli.month.unwrap_or(EarthdateMonthVariant::Alpha), cli.timev.unwrap_or(EarthdateTimeVariant::Metric)));
    }
    cli.command.map(|cmd| match cmd {
        Command::Beat(args) => {
            let precision = args.precision.unwrap_or(2);
            if precision == 0 {
                println!("{}", beat(time) as u32);
            } else {
                println!("{time:.*}", precision as usize, time=beat(time));
            }
        }
    });
}

fn beat(time: DateTime<Local>) -> f32 {
    let timezoned = FixedOffset::east_opt(0).unwrap().from_utc_datetime(&time.naive_utc());
    return timezoned.num_seconds_from_midnight() as f32*1000.0/86400.0;
}

#[derive(Debug, EnumString)]
enum EarthdateVariant {
    Modern, // {year 1-letter-month day . hour minute}
    ModernMetric, // {year 1-letter-month day . metricTime}
    ModernSimple, // {year day-of-year . metricTime}
    ModernSimpleMetric, // {year day-of-year . metricTime}
    Epoch // {year-1 1-letter-month day . hour minute second}
}

#[derive(Debug, EnumString)]
enum EarthdateYearVariant {
    C20, // {year-2000}
    C19, // {year-1900}
    UnixEpoch // {year-1970}
}

#[derive(Debug, EnumString)]
enum EarthdateMonthVariant {
    Numeric, // {numeric month}
    Alpha, // {1-letter-month}
    DayOfYear, // {day-of-year}
    None // {no month}
}

#[derive(Debug, EnumString)]
enum EarthdateTimeVariant {
    Standard, // {hour minute}
    Metric, // {metricTime no decimal places}
    FullStandard, // {hour minute second}
    FullMetric // {metricTime to 2 decimal places}
}

fn earthdate(time: DateTime<Local>, year_variant: EarthdateYearVariant, month_variant: EarthdateMonthVariant, time_variant: EarthdateTimeVariant) -> String {
    let year = match year_variant {
        EarthdateYearVariant::C20 => time.year()-2000,
        EarthdateYearVariant::C19 => time.year()-1900,
        EarthdateYearVariant::UnixEpoch => time.year()-1970,
    };

    let month = match month_variant {
        EarthdateMonthVariant::Numeric => time.format("%m").to_string(),
        EarthdateMonthVariant::Alpha => format!("{}{}", MABV[time.month() as usize].to_string(), time.format("%d").to_string()),
        EarthdateMonthVariant::DayOfYear => time.format("%j").to_string(),
        EarthdateMonthVariant::None => "".to_string(),
    };

    let time_val = match time_variant {
        EarthdateTimeVariant::Standard => time.format("%H%M").to_string(),
        EarthdateTimeVariant::Metric => format!("{}", beat(time) as u32),
        EarthdateTimeVariant::FullStandard => time.format("%H%M%S").to_string(),
        EarthdateTimeVariant::FullMetric => format!("{}", (beat(time)*100.0) as u32),
    };
    return format!("{}{}.{}", year, month, time_val);
}