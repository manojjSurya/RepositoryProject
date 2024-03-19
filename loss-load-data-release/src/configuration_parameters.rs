use clap;
use clap::{App, Arg};
use rbdate::DateParser;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    pub loss_logic_data: String,
    pub gross_loss_id: String,
    pub credit_loss_id: String,
    pub recovery_other_than_credit_loss_id: String,
    pub net_loss_id: String,
    pub external_fraud_id: String,
    pub operational_loss_id: String,
    pub as_on_date: NaiveDate,
    pub output_file: String,
    pub log_file_path: String,
    pub diagnostics_file_path: String,
    pub log_level: String,
    pub is_perf_diagnostics_enabled: bool,
    pub loss_logic_data_sheet_name: String,
}

impl ConfigurationParameters {
    pub fn log_parameters(&self, logger: &Logger) {
        info!(logger, "log_file: {}", self.log_file_path());
        info!(logger, "diagnostics_file: {}", self.diagnostics_file_path());
        info!(logger, "log_level: {}", self.log_level());
        info!(
            logger,
            "The stg.non_sec_exposure_fn is:{}",
            self.loss_logic_data()
        );
        info!(logger, "sheet_name is:{}", self.gross_loss_id());
        info!(logger, "sheet_name is:{}", self.credit_loss_id());
        info!(
            logger,
            "sheet_name is:{}",
            self.recovery_other_than_credit_loss_id()
        );
        info!(logger, "sheet_name is:{}", self.net_loss_id());
        info!(logger, "sheet_name is:{}", self.external_fraud_id());
        info!(logger, "sheet_name is:{}", self.operational_loss_id());
        info!(logger, "sheet_name is:{}", self.as_on_date());
        info!(logger, "output_file_path:{}", self.output_file());
        info!(logger,"sheet_name is:{}",self.loss_logic_data_sheet_name()
        );
    }
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let mut loss_logic_data = matches.value_of("loss_logic_data").unwrap().to_string();
        let mut output_file = matches.value_of("output_file").unwrap().to_string();
        let mut gross_loss_id = matches.value_of("gross_loss_id").unwrap().to_string();
        let mut credit_loss_id = matches.value_of("credit_loss_id").unwrap().to_string();
        let mut recovery_other_than_credit_loss_id = matches
            .value_of("recovery_other_than_credit_loss_id")
            .unwrap()
            .to_string();
        let mut net_loss_id = matches.value_of("net_loss_id").unwrap().to_string();
        let mut external_fraud_id = matches.value_of("external_fraud_id").unwrap().to_string();
        let mut operational_loss_id = matches.value_of("operational_loss_id").unwrap().to_string();
        //let mut as_on_date=matches.value_of("as_on_date").unwrap();
        let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
        let as_on_date = date_parser.parse(
            matches
                .value_of("as_on_date")
                .expect("Error getting `as_on_date`."),
        );
        let mut loss_logic_data_sheet_name = matches
            .value_of("loss_logic_data_sheet_name")
            .unwrap()
            .to_string();

        let log_file_path = matches
            .value_of("log_file")
            .expect("Error getting `log_file_path`.")
            .to_string();
        let diagnostics_file_path = matches
            .value_of("diagnostics_log_file")
            .expect("Error getting `diagnostics_log_file_path`.")
            .to_string();
        let log_level = matches
            .value_of("log_level")
            .expect("Error getting `log_level`.")
            .to_string();
        let is_perf_diagnostics_enabled = matches
            .value_of("perf_diag_flag")
            .expect("Error getting `perf_diag_flag`.")
            .parse::<bool>()
            .expect("Cannot parse `is_perf_diagnostics_enabled` as bool.");

        ConfigurationParameters {
            loss_logic_data,
            gross_loss_id,
            credit_loss_id,
            recovery_other_than_credit_loss_id,
            net_loss_id,
            external_fraud_id,
            operational_loss_id,
            as_on_date,
            log_file_path,
            log_level,
            diagnostics_file_path,
            is_perf_diagnostics_enabled,
            output_file,
            loss_logic_data_sheet_name,
        }
    }
}

// Public getters so an caller can't mutate properties (they're private).
// Also, because users of these properties usually borrow.
impl ConfigurationParameters {
    pub fn log_file_path(&self) -> &str {
        &self.log_file_path
    }
    pub fn diagnostics_file_path(&self) -> &str {
        &self.diagnostics_file_path
    }
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
    pub fn is_perf_diagnostics_enabled(&self) -> bool {
        self.is_perf_diagnostics_enabled
    }
    pub fn loss_logic_data(&self) -> &str {
        &self.loss_logic_data
    }
    pub fn gross_loss_id(&self) -> &str {
        &self.gross_loss_id
    }
    pub fn credit_loss_id(&self) -> &str {
        &self.credit_loss_id
    }
    pub fn recovery_other_than_credit_loss_id(&self) -> &str {
        &self.recovery_other_than_credit_loss_id
    }
    pub fn net_loss_id(&self) -> &str {
        &self.net_loss_id
    }
    pub fn external_fraud_id(&self) -> &str {
        &self.external_fraud_id
    }

    pub fn output_file(&self) -> &str {
        &self.output_file
    }
    pub fn loss_logic_data_sheet_name(&self) -> &str {
        &self.loss_logic_data_sheet_name
    }
    pub fn operational_loss_id(&self) -> &str {
        &self.operational_loss_id
    }
    pub fn as_on_date(&self) -> &NaiveDate {
        &self.as_on_date
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)    
    .about("generating command line arguements for arguements!!")
        .arg(
            Arg::with_name("log_file")
                .long("log-file")
                .value_name("log-file")
                .help("Path to write logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("diagnostics_log_file")
                .long("diagnostics-log-file")
                .value_name("diagnostics-log-file")
                .help("Path to write diagnostics logs.")
                .required(true)
        )
        .arg(
            Arg::with_name("loss_logic_data")
                .long("loss-logic-data")
                .value_name("loss-logic-data")
                .help("Helps to get contents of loss logic data.")
                .required(true)
        )
        .arg(
            Arg::with_name("gross_loss_id")
                .long("gross-loss-id")
                .value_name("gross-loss-id")
                .help("Help write contents to gross loss id.")
                .required(true)
        )
        .arg(
            Arg::with_name("credit_loss_id")
                .long("credit-loss-id")
                .value_name("credit-loss-id")
                .help("Help to write the contents to credit loss id.")
                .required(true)
        )
        .arg(
            Arg::with_name("recovery_other_than_credit_loss_id")
                .long("recovery-other-than-credit-loss-id")
                .value_name("recovery-other-than-credit-loss-id")
                .help("Help to write the recovery loss id.")
                .required(true)
        )
        .arg(
            Arg::with_name("net_loss_id")
                .long("net-loss-id")
                .value_name("net-loss-id")
                .help("Helps write the net loss.")
                .required(true)
        )
        .arg(
            Arg::with_name("external_fraud_id")                                                                                                                                                                                                                       
                .long("external-fraud-id")
                .value_name("external-fraud-id")
                .help("Help get the contents of the master1 file.")
                .required(true)
        )
        
        .arg(
            Arg::with_name("operational_loss_id")
                .long("operational-loss-id")
                .value_name("operational-loss-id")
                .help("Help to  write the operational loss id.")
                .required(true)
        )
        .arg(
            Arg::with_name("output_file")
                .long("output-file")
                .value_name("output-file")
                .help("Helps get the output file path to write contents to text file")
           
                .required(true)
        )
        .arg(
            Arg::with_name("as_on_date")
                .long("as-on-date")
                .value_name("as-on-date")
                .help("Helps get the output file path to write contents to text file")
           
                .required(true)
        )
        .arg(
            Arg::with_name("loss_logic_data_sheet_name")
                .long("finnone-master-sheet-name")
                .value_name("finnone-master-sheet-name")
                .help("Helps get the output file path to write contents to text file")
                .required(true)
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .value_name("LOG LEVEL")
                .possible_values(&["error", "warn", "info", "debug", "trace", "none"])
                .help("Level of diagnostics written to the log file.")
                .required(false)
        )
        .arg(
            Arg::with_name("perf_diag_flag")
                .long("diagnostics-flag")
                .value_name("DIAGNOSTICS FLAG")
                .possible_values(&["true", "false"])
                .help("This flag that decides whether performance diagnostics will be written to the diagnostics log file.")
                .required(false)
        )
      
        .get_matches()
}
