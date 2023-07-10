#![allow(non_camel_case_types)] // allowing All Caps names for constants

use std::fmt;
use std::ffi::NulError;

#[derive(Debug)]
#[derive(PartialEq)]
#[repr(i32)]
pub enum LexActivatorStatusCode {
    LA_OK = 0,
    LA_FAIL = 1,
    LA_EXPIRED = 20,
    LA_SUSPENDED =21,
    LA_GRACE_PERIOD_OVER = 22,
    LA_TRIAL_EXPIRED = 25,
    LA_LOCAL_TRIAL_EXPIRED = 26,
    LA_RELEASE_UPDATE_AVAILABLE = 30,
    LA_RELEASE_UPDATE_NOT_AVAILABLE = 31,
    LA_RELEASE_UPDATE_AVAILABLE_NOT_ALLOWED = 32,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[repr(i32)]
pub enum LexActivatorErrorCode {
    LA_E_FILE_PATH = 40,
    LA_E_PRODUCT_FILE = 41,
    LA_E_PRODUCT_DATA = 42,
    LA_E_PRODUCT_ID = 43,
    LA_E_SYSTEM_PERMISSION = 44,
    LA_E_FILE_PERMISSION = 45,
    LA_E_WMIC = 46,
    LA_E_TIME = 47,
    LA_E_INET = 48,
    LA_E_NET_PROXY = 49,
    LA_E_HOST_URL = 50,
    LA_E_BUFFER_SIZE = 51,
    LA_E_APP_VERSION_LENGTH = 52,
    LA_E_REVOKED = 53,
    LA_E_LICENSE_KEY = 54,
    LA_E_LICENSE_TYPE = 55,
    LA_E_OFFLINE_RESPONSE_FILE = 56,
    LA_E_OFFLINE_RESPONSE_FILE_EXPIRED = 57,
    LA_E_ACTIVATION_LIMIT = 58,
    LA_E_ACTIVATION_NOT_FOUND = 59,
    LA_E_DEACTIVATION_LIMIT = 60,
    LA_E_TRIAL_NOT_ALLOWED = 61,
    LA_E_TRIAL_ACTIVATION_LIMIT = 62,
    LA_E_MACHINE_FINGERPRINT = 63,
    LA_E_METADATA_KEY_LENGTH = 64,
    LA_E_METADATA_VALUE_LENGTH = 65,
    LA_E_ACTIVATION_METADATA_LIMIT = 66,
    LA_E_TRIAL_ACTIVATION_METADATA_LIMIT = 67,
    LA_E_METADATA_KEY_NOT_FOUND = 68,
    LA_E_TIME_MODIFIED = 69,
    LA_E_RELEASE_VERSION_FORMAT = 70,
    LA_E_AUTHENTICATION_FAILED = 71,
    LA_E_METER_ATTRIBUTE_NOT_FOUND = 72,
    LA_E_METER_ATTRIBUTE_USES_LIMIT_REACHED = 73,
    LA_E_CUSTOM_FINGERPRINT_LENGTH = 74,
    LA_E_PRODUCT_VERSION_NOT_LINKED = 75,
    LA_E_FEATURE_FLAG_NOT_FOUND = 76,
    LA_E_RELEASE_VERSION_NOT_ALLOWED = 77,
    LA_E_RELEASE_PLATFORM_LENGTH = 78,
    LA_E_RELEASE_CHANNEL_LENGTH = 79,
    LA_E_VM = 80,
    LA_E_COUNTRY = 81,
    LA_E_IP = 82,
    LA_E_CONTAINER = 83,
    LA_E_RELEASE_VERSION = 84,
    LA_E_RELEASE_PLATFORM = 85,
    LA_E_RELEASE_CHANNEL = 86,
    LA_E_RATE_LIMIT = 90,
    LA_E_SERVER = 91,
    LA_E_CLIENT = 92
}

impl From<i32> for LexActivatorStatusCode {
    fn from(code: i32) -> Self {
        match code {
            0 => LexActivatorStatusCode::LA_OK,
            1 => LexActivatorStatusCode::LA_FAIL,
            20 => LexActivatorStatusCode::LA_EXPIRED,
            21 => LexActivatorStatusCode::LA_SUSPENDED,
            22 => LexActivatorStatusCode::LA_GRACE_PERIOD_OVER,
            25 => LexActivatorStatusCode::LA_TRIAL_EXPIRED,
            26 => LexActivatorStatusCode::LA_LOCAL_TRIAL_EXPIRED,
            30 => LexActivatorStatusCode::LA_RELEASE_UPDATE_AVAILABLE,
            31 => LexActivatorStatusCode::LA_RELEASE_UPDATE_NOT_AVAILABLE,
            32 => LexActivatorStatusCode::LA_RELEASE_UPDATE_AVAILABLE_NOT_ALLOWED,
            _ => todo!(),
        }  
    }
}

impl From<i32> for LexActivatorErrorCode {
    fn from(code: i32) -> Self {
        match code {
            40 => LexActivatorErrorCode::LA_E_FILE_PATH,
            41 => LexActivatorErrorCode::LA_E_PRODUCT_FILE,
            42 => LexActivatorErrorCode::LA_E_PRODUCT_DATA,
            43 => LexActivatorErrorCode::LA_E_PRODUCT_ID,
            44 => LexActivatorErrorCode::LA_E_SYSTEM_PERMISSION,
            45 => LexActivatorErrorCode::LA_E_FILE_PERMISSION,
            46 => LexActivatorErrorCode::LA_E_WMIC,
            47 => LexActivatorErrorCode::LA_E_TIME,
            48 => LexActivatorErrorCode::LA_E_INET,
            49 => LexActivatorErrorCode::LA_E_NET_PROXY,
            50 => LexActivatorErrorCode::LA_E_HOST_URL,
            51 => LexActivatorErrorCode::LA_E_BUFFER_SIZE,
            52 => LexActivatorErrorCode::LA_E_APP_VERSION_LENGTH,
            53 => LexActivatorErrorCode::LA_E_REVOKED,
            54 => LexActivatorErrorCode::LA_E_LICENSE_KEY,
            55 => LexActivatorErrorCode::LA_E_LICENSE_TYPE,
            56 => LexActivatorErrorCode::LA_E_OFFLINE_RESPONSE_FILE,
            57 => LexActivatorErrorCode::LA_E_OFFLINE_RESPONSE_FILE_EXPIRED,
            58 => LexActivatorErrorCode::LA_E_ACTIVATION_LIMIT,
            59 => LexActivatorErrorCode::LA_E_ACTIVATION_NOT_FOUND,
            60 => LexActivatorErrorCode::LA_E_DEACTIVATION_LIMIT,
            61 => LexActivatorErrorCode::LA_E_TRIAL_NOT_ALLOWED,
            62 => LexActivatorErrorCode::LA_E_TRIAL_ACTIVATION_LIMIT,
            63 => LexActivatorErrorCode::LA_E_MACHINE_FINGERPRINT,
            64 => LexActivatorErrorCode::LA_E_METADATA_KEY_LENGTH,
            65 => LexActivatorErrorCode::LA_E_METADATA_VALUE_LENGTH,
            66 => LexActivatorErrorCode::LA_E_ACTIVATION_METADATA_LIMIT,
            67 => LexActivatorErrorCode::LA_E_TRIAL_ACTIVATION_METADATA_LIMIT,
            68 => LexActivatorErrorCode::LA_E_METADATA_KEY_NOT_FOUND,
            69 => LexActivatorErrorCode::LA_E_TIME_MODIFIED,
            70 => LexActivatorErrorCode::LA_E_RELEASE_VERSION_FORMAT,
            71 => LexActivatorErrorCode::LA_E_AUTHENTICATION_FAILED,
            72 => LexActivatorErrorCode::LA_E_METER_ATTRIBUTE_NOT_FOUND,
            73 => LexActivatorErrorCode::LA_E_METER_ATTRIBUTE_USES_LIMIT_REACHED,
            74 => LexActivatorErrorCode::LA_E_CUSTOM_FINGERPRINT_LENGTH,
            75 => LexActivatorErrorCode::LA_E_PRODUCT_VERSION_NOT_LINKED,
            76 => LexActivatorErrorCode::LA_E_FEATURE_FLAG_NOT_FOUND,
            77 => LexActivatorErrorCode::LA_E_RELEASE_VERSION_NOT_ALLOWED,
            78 => LexActivatorErrorCode::LA_E_RELEASE_PLATFORM_LENGTH,
            79 => LexActivatorErrorCode::LA_E_RELEASE_CHANNEL_LENGTH,
            80 => LexActivatorErrorCode::LA_E_VM,
            81 => LexActivatorErrorCode::LA_E_COUNTRY,
            82 => LexActivatorErrorCode::LA_E_IP,
            83 => LexActivatorErrorCode::LA_E_CONTAINER,
            84 => LexActivatorErrorCode::LA_E_RELEASE_VERSION,
            85 => LexActivatorErrorCode::LA_E_RELEASE_PLATFORM,
            86 => LexActivatorErrorCode::LA_E_RELEASE_CHANNEL,
            90 => LexActivatorErrorCode::LA_E_RATE_LIMIT,
            91 => LexActivatorErrorCode::LA_E_SERVER,
            92 => LexActivatorErrorCode::LA_E_CLIENT,
            _ => todo!(),
            // Add more mappings as needed
        }
    }
}

impl fmt::Display for LexActivatorStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexActivatorStatusCode::LA_OK => write!(f, "{} Success code.", LexActivatorStatusCode::LA_OK as i32),
            LexActivatorStatusCode::LA_FAIL => write!(f, "{} Failure code.", LexActivatorStatusCode::LA_FAIL as i32),
            LexActivatorStatusCode::LA_EXPIRED => write!(f, "{} The license has expired or system time has been tampered with. Ensure your date and time settings are correct.", LexActivatorStatusCode::LA_EXPIRED as i32),
            LexActivatorStatusCode::LA_SUSPENDED => write!(f, "{} The license has been suspended.", LexActivatorStatusCode::LA_SUSPENDED as i32),
            LexActivatorStatusCode::LA_GRACE_PERIOD_OVER => write!(f, "{} The grace period for server sync is over.", LexActivatorStatusCode::LA_GRACE_PERIOD_OVER as i32),
            LexActivatorStatusCode::LA_TRIAL_EXPIRED => write!(f, "{} The trial has expired or system time has been tampered with. Ensure your date and time settings are correct.", LexActivatorStatusCode::LA_TRIAL_EXPIRED as i32),
            LexActivatorStatusCode::LA_LOCAL_TRIAL_EXPIRED => write!(f, "{} The local trial has expired or system time has been tampered with. Ensure your date and time settings are correct.", LexActivatorStatusCode::LA_LOCAL_TRIAL_EXPIRED as i32),
            LexActivatorStatusCode::LA_RELEASE_UPDATE_AVAILABLE => write!(f, "{} A new update is available for the product. This means a new release has been published for the product.", LexActivatorStatusCode::LA_RELEASE_UPDATE_AVAILABLE as i32),
            LexActivatorStatusCode::LA_RELEASE_UPDATE_NOT_AVAILABLE => write!(f, "{} No new update is available for the product. The current version is latest.", LexActivatorStatusCode::LA_RELEASE_UPDATE_NOT_AVAILABLE as i32),
            LexActivatorStatusCode::LA_RELEASE_UPDATE_AVAILABLE_NOT_ALLOWED => write!(f, "{} The update available is not allowed for this license.", LexActivatorStatusCode::LA_RELEASE_UPDATE_AVAILABLE_NOT_ALLOWED as i32),
        }
    }
}     

impl fmt::Display for LexActivatorErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexActivatorErrorCode::LA_E_FILE_PATH => write!(f, "{} Invalid file path.", LexActivatorErrorCode::LA_E_FILE_PATH as i32),
            LexActivatorErrorCode::LA_E_PRODUCT_FILE => write!(f, "{} Invalid or corrupted product file.", LexActivatorErrorCode::LA_E_PRODUCT_FILE as i32),
            LexActivatorErrorCode::LA_E_PRODUCT_DATA => write!(f, "{} Invalid product data.", LexActivatorErrorCode::LA_E_PRODUCT_DATA as i32),
            LexActivatorErrorCode::LA_E_PRODUCT_ID => write!(f, "{} The product id is incorrect.", LexActivatorErrorCode::LA_E_PRODUCT_ID as i32),
            LexActivatorErrorCode::LA_E_SYSTEM_PERMISSION => write!(f, "{} Insufficent system permissions.", LexActivatorErrorCode::LA_E_SYSTEM_PERMISSION as i32),
            LexActivatorErrorCode::LA_E_FILE_PERMISSION => write!(f, "{} No permission to write to file.", LexActivatorErrorCode::LA_E_FILE_PERMISSION as i32),
            LexActivatorErrorCode::LA_E_WMIC => write!(f, "{} Fingerprint couldn't be generated because Windows Management Instrumentation (WMI) service has been disabled.", LexActivatorErrorCode::LA_E_WMIC as i32),
            LexActivatorErrorCode::LA_E_TIME => write!(f, "{} The difference between the network time and the system time is more than allowed clock offset.", LexActivatorErrorCode::LA_E_TIME as i32),
            LexActivatorErrorCode::LA_E_INET => write!(f, "{} Failed to connect to the server due to network error.", LexActivatorErrorCode::LA_E_INET as i32),
            LexActivatorErrorCode::LA_E_NET_PROXY => write!(f, "{} Invalid network proxy.", LexActivatorErrorCode::LA_E_NET_PROXY as i32),
            LexActivatorErrorCode::LA_E_HOST_URL => write!(f, "{} Invalid Cryptlex host url.", LexActivatorErrorCode::LA_E_HOST_URL as i32),
            LexActivatorErrorCode::LA_E_BUFFER_SIZE => write!(f, "{} The buffer size was smaller than required.", LexActivatorErrorCode::LA_E_BUFFER_SIZE as i32),
            LexActivatorErrorCode::LA_E_APP_VERSION_LENGTH => write!(f, "{} App version length is more than characters.", LexActivatorErrorCode::LA_E_APP_VERSION_LENGTH as i32),
            LexActivatorErrorCode::LA_E_REVOKED => write!(f, "{} The license has been revoked.", LexActivatorErrorCode::LA_E_REVOKED as i32),
            LexActivatorErrorCode::LA_E_LICENSE_KEY => write!(f, "{} Invalid license key.", LexActivatorErrorCode::LA_E_LICENSE_KEY as i32),
            LexActivatorErrorCode::LA_E_LICENSE_TYPE => write!(f, "{} Invalid license type. Make sure floating license is not being used.", LexActivatorErrorCode::LA_E_LICENSE_TYPE as i32),
            LexActivatorErrorCode::LA_E_OFFLINE_RESPONSE_FILE => write!(f, "{} Invalid offline activation response file.", LexActivatorErrorCode::LA_E_OFFLINE_RESPONSE_FILE as i32),
            LexActivatorErrorCode::LA_E_OFFLINE_RESPONSE_FILE_EXPIRED => write!(f, "{} The offline activation response has expired.", LexActivatorErrorCode::LA_E_OFFLINE_RESPONSE_FILE_EXPIRED as i32),
            LexActivatorErrorCode::LA_E_ACTIVATION_LIMIT => write!(f, "{} The license has reached it's allowed activations limit.", LexActivatorErrorCode::LA_E_ACTIVATION_LIMIT as i32),
            LexActivatorErrorCode::LA_E_ACTIVATION_NOT_FOUND => write!(f, "{} The license activation was deleted on the server.", LexActivatorErrorCode::LA_E_ACTIVATION_NOT_FOUND as i32),
            LexActivatorErrorCode::LA_E_DEACTIVATION_LIMIT => write!(f, "{} The license has reached it's allowed deactivations limit.", LexActivatorErrorCode::LA_E_DEACTIVATION_LIMIT as i32),
            LexActivatorErrorCode::LA_E_TRIAL_NOT_ALLOWED => write!(f, "{} Trial not allowed for the product.", LexActivatorErrorCode::LA_E_TRIAL_NOT_ALLOWED as i32),
            LexActivatorErrorCode::LA_E_TRIAL_ACTIVATION_LIMIT => write!(f, "{} Your account has reached it's trial activations limit and trial not allowed for the product.", LexActivatorErrorCode::LA_E_TRIAL_ACTIVATION_LIMIT as i32),
            LexActivatorErrorCode::LA_E_MACHINE_FINGERPRINT => write!(f, "{} Machine fingerprint has changed since activation.", LexActivatorErrorCode::LA_E_MACHINE_FINGERPRINT as i32),
            LexActivatorErrorCode::LA_E_METADATA_KEY_LENGTH => write!(f, "{} Metadata key length is more than 256 characters.", LexActivatorErrorCode::LA_E_METADATA_KEY_LENGTH as i32),
            LexActivatorErrorCode::LA_E_METADATA_VALUE_LENGTH => write!(f, "{} Metadata value length is more than 256 characters.", LexActivatorErrorCode::LA_E_METADATA_VALUE_LENGTH as i32),
            LexActivatorErrorCode::LA_E_ACTIVATION_METADATA_LIMIT => write!(f, "{} The license has reached it's metadata fields limit.", LexActivatorErrorCode::LA_E_ACTIVATION_METADATA_LIMIT as i32),
            LexActivatorErrorCode::LA_E_TRIAL_ACTIVATION_METADATA_LIMIT => write!(f, "{} The trial has reached it's metadata fields limit.", LexActivatorErrorCode::LA_E_TRIAL_ACTIVATION_METADATA_LIMIT as i32),
            LexActivatorErrorCode::LA_E_METADATA_KEY_NOT_FOUND => write!(f, "{} The metadata key does not exist.", LexActivatorErrorCode::LA_E_METADATA_KEY_NOT_FOUND as i32),
            LexActivatorErrorCode::LA_E_TIME_MODIFIED => write!(f, "{} The system time has been tampered (backdated).", LexActivatorErrorCode::LA_E_TIME_MODIFIED as i32),
            LexActivatorErrorCode::LA_E_RELEASE_VERSION_FORMAT => write!(f, "{} Invalid version format.", LexActivatorErrorCode::LA_E_RELEASE_VERSION_FORMAT as i32),
            LexActivatorErrorCode::LA_E_AUTHENTICATION_FAILED => write!(f, "{} Incorrect email or password.", LexActivatorErrorCode::LA_E_AUTHENTICATION_FAILED as i32),
            LexActivatorErrorCode::LA_E_METER_ATTRIBUTE_NOT_FOUND => write!(f, "{} The meter attribute does not exist.", LexActivatorErrorCode::LA_E_METER_ATTRIBUTE_NOT_FOUND as i32),
            LexActivatorErrorCode::LA_E_METER_ATTRIBUTE_USES_LIMIT_REACHED => write!(f, "{} The meter attribute has reached it's usage limit.", LexActivatorErrorCode::LA_E_METER_ATTRIBUTE_USES_LIMIT_REACHED as i32),
            LexActivatorErrorCode::LA_E_CUSTOM_FINGERPRINT_LENGTH => write!(f, "{} Custom device fingerprint length is less than 64 characters or more than 256 characters.", LexActivatorErrorCode::LA_E_CUSTOM_FINGERPRINT_LENGTH as i32),
            LexActivatorErrorCode::LA_E_PRODUCT_VERSION_NOT_LINKED => write!(f, "{} No product version is linked with the license.", LexActivatorErrorCode::LA_E_PRODUCT_VERSION_NOT_LINKED as i32),
            LexActivatorErrorCode::LA_E_FEATURE_FLAG_NOT_FOUND => write!(f, "{} The product version feature flag does not exist.", LexActivatorErrorCode::LA_E_FEATURE_FLAG_NOT_FOUND as i32),
            LexActivatorErrorCode::LA_E_RELEASE_VERSION_NOT_ALLOWED => write!(f, "{} The release version is not allowed.", LexActivatorErrorCode::LA_E_RELEASE_VERSION_NOT_ALLOWED as i32),
            LexActivatorErrorCode::LA_E_RELEASE_PLATFORM_LENGTH => write!(f, "{} Release platform length is more than 256 characters.", LexActivatorErrorCode::LA_E_RELEASE_PLATFORM_LENGTH as i32),
            LexActivatorErrorCode::LA_E_RELEASE_CHANNEL_LENGTH => write!(f, "{} Release channel length is more than 256 characters.", LexActivatorErrorCode::LA_E_RELEASE_CHANNEL_LENGTH as i32),
            LexActivatorErrorCode::LA_E_VM => write!(f, "{} Application is running inside virtual machine / hypervisor and activation has been disallowed in the VM.", LexActivatorErrorCode::LA_E_VM as i32),
            LexActivatorErrorCode::LA_E_COUNTRY => write!(f, "{} Country is not allowed.", LexActivatorErrorCode::LA_E_COUNTRY as i32),
            LexActivatorErrorCode::LA_E_IP => write!(f, "{} IP address is not allowed.", LexActivatorErrorCode::LA_E_IP as i32),
            LexActivatorErrorCode::LA_E_CONTAINER => write!(f, "{} Application is being run inside a container and activation has been disallowed in the container.", LexActivatorErrorCode::LA_E_CONTAINER as i32),
            LexActivatorErrorCode::LA_E_RELEASE_VERSION => write!(f, "{} Invalid release version. Make sure the release version uses the following formats: x.x, x.x.x, x.x.x.x (where x is a number).", LexActivatorErrorCode::LA_E_RELEASE_VERSION as i32),
            LexActivatorErrorCode::LA_E_RELEASE_PLATFORM => write!(f, "{} Release platform not set.", LexActivatorErrorCode::LA_E_RELEASE_PLATFORM as i32),
            LexActivatorErrorCode::LA_E_RELEASE_CHANNEL => write!(f, "{} Release channel not set.", LexActivatorErrorCode::LA_E_RELEASE_CHANNEL as i32),
            LexActivatorErrorCode::LA_E_RATE_LIMIT => write!(f, "{} Rate limit for API has reached, try again later.", LexActivatorErrorCode::LA_E_RATE_LIMIT as i32),
            LexActivatorErrorCode::LA_E_SERVER => write!(f, "{} Server error.", LexActivatorErrorCode::LA_E_SERVER as i32),
            LexActivatorErrorCode::LA_E_CLIENT => write!(f, "{} Client error.", LexActivatorErrorCode::LA_E_CLIENT as i32),
        }
    }
}

impl From<NulError> for LexActivatorErrorCode {
    fn from(_: NulError) -> Self {
        LexActivatorErrorCode::LA_E_CLIENT  
    }
}