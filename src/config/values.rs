use crate::{namespace, var};
use toml::Value;
use crate::config::ConfigurationError;
namespace!(
    name: "logger",
    tpe: Logger,
    body:
        log_level: LogLevel,
        log_warnings: LogWarnings,
        log_exceptions: LogExceptions,
        log_to_file: LogToFile,
        log_file_path: LogFilePath,
        log_file_level: LogFileLevel,
        log_file_format: LogFileFormat,
        log_file_encoding: LogFileEncoding,
);

namespace!(
    name: "console",
    tpe: Console,
    body:
        unicode_output: UnicodeOutput,
        use_color: UseColor,
        max_lines: MaxLines,
        max_width: MaxWidth,
);

namespace!(
    name: "visualization.wcsaxes",
    tpe: Visualization,
    body:
        coordinate_range_samples: CoordinateRangeSamples,
        frame_boundary_samples: FrameBoundarySamples,
        grid_samples: GridSamples,
        contour_grid_samples: ContourGridSamples,
);

namespace!(
    name: "utils.iers",
    tpe: Iers,
    body:
        auto_download: AutoDownload,
        auto_max_age: AutoMaxAge,
        iers_auto_url: IersAutoUrl,
        iers_auto_url_mirror: IersAutoUrlMirror,
        remote_timeout: RemoteTimeout,
        iers_degraded_accuracy: IersDegradedAccuracy,
        system_leap_second_file: SystemLeapSecondFile,
        iers_leap_second_auto_url: IersLeapSecondAutoUrl,
);


namespace!(
    name: "utils.data",
    tpe: Data,
    body:
        allow_internet: AllowInternet,
        compute_hash_block_size: ComputeHashBlockSize,
        data_url: DataUrl,
        data_url_mirror: DataUrlMirror,
        default_http_user_agent: DefaultHttpUserAgent,
        delete_temporary_downloads_at_exit: DeleteTemporaryDownloadsAtExit,
        download_block_size: DownloadBlockSize,
        data_query_remote_timeout: DataQueryRemoteTimeout,
);


var!(
    name: LogLevel,
    type: String,
    dsc: "# The level of the logger",
    default: "INFO".to_string()
);

var!(
    name:  LogWarnings,
    type: bool,
    dsc: "# Whether to log `warnings.warn` calls.",
    default: true
);

var!(
    name: LogExceptions,
    type: bool,
    dsc: "# Whether to log exceptions before raising them.",
    default: false
);

var!(
    name: LogToFile,
    type: bool,
    dsc: "# Whether to always log messages to a log file.",
    default: false
);

var!(
    name: LogFilePath,
    type: String,
    dsc: r#"# The file to log messages to.
# If empty string is given, it defaults to a file ``'astropy.log'`` in the astropy config directory."#,
    default: "".to_string()
);

var!(
    name: LogFileLevel,
    type: String,
    dsc: "# Threshold for logging messages to `log_file_path`.",
    default: "INFO".to_string()
);

var!(
    name: LogFileFormat,
    type: String,
    dsc: "# Format for log file entries.",
    default: "%(asctime)r, %(origin)r, %(levelname)r, %(message)r".to_string()
);

var!(
    name: LogFileEncoding,
    type: String,
    dsc: r#"# The encoding (e.g., UTF-8) to use for the log file.
# If empty string is given, it defaults to the platform-preferred encoding."#,
    default: "".to_string()
);

var!(
    name: AllowInternet,
    type: bool,
    dsc: "# If False, prevents any attempt to download from Internet.",
    default: true
);
var!(
    name: ComputeHashBlockSize,
    type: i32,
    dsc: "# Block size for computing file hashes.",
    default: 65536
);

var!(
    name: DataUrl,
    type: String,
    dsc: "# Primary URL for astropy remote data site.",
    default: "http://data.astropy.org/".to_string()
);

var!(
    name: DataUrlMirror,
    type: String,
    dsc: "# Mirror URL for astropy remote data site.",
    default: "http://www.astropy.org/astropy-data/".to_string()
);

var!(
    name: DefaultHttpUserAgent,
    type: String,
    dsc: r#"# Default User-Agent for HTTP request headers.
# This can be overwritten for a particular call via http_headers option, where available.
# This only provides the default value when not set by https_headers."#,
    default: "astropy".to_string()
);

var!(
    name: DeleteTemporaryDownloadsAtExit,
    type: bool,
    dsc: r#"# If True, temporary download files created
# when the cache is inaccessible will be deleted at the end of the python session."#,
    default: true
);

var!(
    name: DownloadBlockSize,
    type: i32,
    dsc: "# Number of bytes of remote data to download per step.",
    default: 65536
);

var!(
    name: DataQueryRemoteTimeout,
    type: f64,
    dsc: "# Time to wait for remote data queries (in seconds).",
    default: 10.0
);


var!(
    name: AutoDownload,
    type: bool,
    dsc: r#"#Enable auto-downloading of the latest IERS data.
# If set to False then the local IERS-B file will be used (even
# if the full IERS file with predictions was already downloaded and cached).
# This parameter also controls whether internet resources will be queried
# to update the leap second table if the installed version is out of date. Default is True."#,
    default: true
);

var!(
    name: AutoMaxAge,
    type: f32,
    dsc: r#"# Maximum age (days) of predictive data before auto-downloading.
# See 'Auto refresh behavior' in astropy.utils.iers documentation for details. Default is 30."#,
    default: 30.0
);

var!(
    name: IersAutoUrl,
    type: String,
    dsc: "# URL for auto-downloading IERS file data.",
    default: "https://datacenter.iers.org/data/9/finals2000A.all".to_string()
);

var!(
    name: IersAutoUrlMirror,
    type: String,
    dsc: "# Mirror URL for auto-downloading IERS file data.",
    default: "https://maia.usno.navy.mil/ser7/finals2000A.all".to_string()
);

var!(
    name: RemoteTimeout,
    type: f32,
    dsc: "# Remote timeout downloading IERS file data (seconds).",
    default: 10.0
);

var!(
    name: IersDegradedAccuracy,
    type: String,
    dsc: r#"# IERS behavior if the range of available IERS data does not cover
# the times when converting time scales, potentially leading to degraded accuracy."#,
    default: "error".to_string()
);

var!(
    name: SystemLeapSecondFile,
    type: String,
    dsc: "# System file with leap seconds.",
    default: "".to_string()
);

var!(
    name: IersLeapSecondAutoUrl,
    type: String,
    dsc: "# URL for auto-downloading leap seconds.",
    default: "https://hpiers.obspm.fr/iers/bul/bulc/Leap_Second.dat".to_string()
);


var!(
    name: CoordinateRangeSamples,
    type: i32,
    dsc: "# The number of samples along each image axis when determining the range of coordinates in a plot.",
    default: 50
);

var!(
    name: FrameBoundarySamples,
    type: i32,
    dsc: "# How many points to sample along the axes when determining tick locations.",
    default: 1000
);

var!(
    name: GridSamples,
    type: i32,
    dsc: "# How many points to sample along grid lines.",
    default: 1000
);

var!(
    name: ContourGridSamples,
    type: i32,
    dsc: "# The grid size to use when drawing a grid using contours",
    default: 200
);



var!(
    name: UnicodeOutput,
    type: bool,
    dsc: "# When True, use Unicode characters when outputting values, and displaying widgets at the console",
    default: false
);

var!(
    name: UseColor,
    type: bool,
    dsc: "# When True, use ANSI color escape sequences when writing to the console",
    default: true
);

var!(
    name: MaxLines,
    type: i32,
    dsc: r#"# Maximum number of lines in the display of pretty-printed objects.
# If not provided, try to determine automatically from the terminal size.
# Negative numbers mean no limit."#,
    default: -1
);

var!(
    name: MaxWidth,
    type: i32,
    dsc: r#"# Maximum number of characters per line in the display of pretty-printed objects.
# If not provided, try to determine automatically from the terminal size.
# Negative numbers mean no limit."#,
    default: -1
);