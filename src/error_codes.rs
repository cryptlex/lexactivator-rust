#![allow(non_camel_case_types)] // allowing All Caps names for constants

use std::fmt;
use std::ffi::NulError;

#[derive(Debug)]
#[derive(PartialEq)]
#[repr(i32)]
pub enum LexActivatorStatus {
    /// Success code.
    LA_OK = 0,
    /// Failure code.
    LA_FAIL = 1,
    /// The license has expired or system time has been tampered with. Ensure your date and time settings are correct.
    LA_EXPIRED = 20,
    /// The license has been suspended.
    LA_SUSPENDED =21,
    /// The grace period for server sync is over.
    LA_GRACE_PERIOD_OVER = 22,
    /// The trial has expired or system time has been tampered with. Ensure your date and time settings are correct.
    LA_TRIAL_EXPIRED = 25,
    /// The local trial has expired or system time has been tampered with. Ensure your date and time settings are correct.
    LA_LOCAL_TRIAL_EXPIRED = 26,
    /// A new update is available for the product. This means a new release has been published for the product.
    LA_RELEASE_UPDATE_AVAILABLE = 30,
    /// No new update is available for the product. The current version is latest.
    LA_RELEASE_UPDATE_NOT_AVAILABLE = 31,
    /// The update available is not allowed for this license.
    LA_RELEASE_UPDATE_AVAILABLE_NOT_ALLOWED = 32,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[repr(i32)]
pub enum LexActivatorError {
    /// Failure code.
    LA_FAIL = 1,
    /// Invalid file path.
    LA_E_FILE_PATH = 40,
    /// Invalid or corrupted product file.
    LA_E_PRODUCT_FILE = 41,
    /// Invalid product data.
    LA_E_PRODUCT_DATA = 42,
    /// The product id is incorrect.
    LA_E_PRODUCT_ID = 43,
    /// Insufficient system permissions. Occurs when LA_SYSTEM flag is used but application is not run with admin privileges.
    LA_E_SYSTEM_PERMISSION = 44,
    /// No permission to write to file.
    LA_E_FILE_PERMISSION = 45,
    /// Fingerprint couldn't be generated because Windows Management Instrumentation (WMI) service has been disabled. This error is specific to Windows only.
    LA_E_WMIC = 46,
    /// The difference between the network time and the system time is more than allowed clock offset.
    LA_E_TIME = 47,
    /// Failed to connect to the server due to network error.
    LA_E_INET = 48,
    /// Invalid network proxy.
    LA_E_NET_PROXY = 49,
    /// Invalid Cryptlex host url.
    LA_E_HOST_URL = 50,
    /// The buffer size was smaller than required.
    LA_E_BUFFER_SIZE = 51,
    /// App version length is more than 256 characters.
    LA_E_APP_VERSION_LENGTH = 52,
    /// The license has been revoked.
    LA_E_REVOKED = 53,
    /// Invalid license key.
    LA_E_LICENSE_KEY = 54,
    /// Invalid license type. Make sure floating license is not being used.
    LA_E_LICENSE_TYPE = 55,
    /// Invalid offline activation response file.
    LA_E_OFFLINE_RESPONSE_FILE = 56,
    /// The offline activation response has expired.
    LA_E_OFFLINE_RESPONSE_FILE_EXPIRED = 57,
    /// The license has reached it's allowed activations limit.
    LA_E_ACTIVATION_LIMIT = 58,
    /// The license activation was deleted on the server.
    LA_E_ACTIVATION_NOT_FOUND = 59,
    /// The license has reached it's allowed deactivations limit.
    LA_E_DEACTIVATION_LIMIT = 60,
    /// Trial not allowed for the product.
    LA_E_TRIAL_NOT_ALLOWED = 61,
    /// Your account has reached it's trial activations limit.
    LA_E_TRIAL_ACTIVATION_LIMIT = 62,
    /// Machine fingerprint has changed since activation.
    LA_E_MACHINE_FINGERPRINT = 63,
    /// Metadata key length is more than 256 characters.
    LA_E_METADATA_KEY_LENGTH = 64,
    /// Metadata value length is more than 4096 characters.
    LA_E_METADATA_VALUE_LENGTH = 65,
    /// The license has reached it's metadata fields limit.
    LA_E_ACTIVATION_METADATA_LIMIT = 66,
    /// The trial has reached it's metadata fields limit.
    LA_E_TRIAL_ACTIVATION_METADATA_LIMIT = 67,
    /// The metadata key does not exist.
    LA_E_METADATA_KEY_NOT_FOUND = 68,
    /// The system time has been tampered (backdated).
    LA_E_TIME_MODIFIED = 69,
    /// Invalid version format.
    LA_E_RELEASE_VERSION_FORMAT = 70,
    /// Incorrect email or password.
    LA_E_AUTHENTICATION_FAILED = 71,
    /// The meter attribute does not exist.
    LA_E_METER_ATTRIBUTE_NOT_FOUND = 72,
    /// The meter attribute has reached it's usage limit.
    LA_E_METER_ATTRIBUTE_USES_LIMIT_REACHED = 73,
    /// Custom device fingerprint length is less than 64 characters or more than 256 characters.
    LA_E_CUSTOM_FINGERPRINT_LENGTH = 74,
    /// No product version is linked with the license.
    LA_E_PRODUCT_VERSION_NOT_LINKED = 75,
    /// The product version feature flag does not exist.
    LA_E_FEATURE_FLAG_NOT_FOUND = 76,
    /// The release version is not allowed.
    LA_E_RELEASE_VERSION_NOT_ALLOWED = 77,
    /// Release platform length is more than 256 characters.
    LA_E_RELEASE_PLATFORM_LENGTH = 78,
    /// Release channel length is more than 256 characters.
    LA_E_RELEASE_CHANNEL_LENGTH = 79,
    /// Application is being run inside a virtual machine / hypervisor, and activation has been disallowed in the VM.
    LA_E_VM = 80,
    /// Country is not allowed.
    LA_E_COUNTRY = 81,
    /// IP address is not allowed.
    LA_E_IP = 82,
    /// Application is being run inside a container and activation has been disallowed in the container.
    LA_E_CONTAINER = 83,
    /// Invalid release version. Make sure the release version uses the following formats: x.x, x.x.x, x.x.x.x (where x is a number).
    LA_E_RELEASE_VERSION = 84,
    /// Release platform not set.
    LA_E_RELEASE_PLATFORM = 85,
    /// Release channel not set.
    LA_E_RELEASE_CHANNEL = 86,
    /// The user is not authenticated.
    LA_E_USER_NOT_AUTHENTICATED = 87,
    /// The two-factor authentication code for the user authentication is missing.
    LA_E_TWO_FACTOR_AUTHENTICATION_CODE_MISSING = 88,
    /// The two-factor authentication code provided by the user is invalid.
    LA_E_TWO_FACTOR_AUTHENTICATION_CODE_INVALID = 89,
    /// Rate limit for API has reached, try again later.
    LA_E_RATE_LIMIT = 90,
    /// Server error.
    LA_E_SERVER = 91,
    /// Client error.
    LA_E_CLIENT = 92,
    /// Invalid account ID.
    LA_E_ACCOUNT_ID = 93,
    /// The user account has been temporarily locked for 5 mins due to 5 failed attempts.
    LA_E_LOGIN_TEMPORARILY_LOCKED = 100,
    /// Invalid authentication ID token.
    LA_E_AUTHENTICATION_ID_TOKEN_INVALID = 101,
    /// OIDC SSO is not enabled.
    LA_E_OIDC_SSO_NOT_ENABLED = 102,
    /// The allowed users for this account has reached its limit.
    LA_E_USERS_LIMIT_REACHED = 103,
    /// OS user has changed since activation and the license is user-locked.
    LA_E_OS_USER = 104,
    /// Invalid permission flag.
    LA_E_INVALID_PERMISSION_FLAG = 105,
    /// The free plan has reached its activation limit.
    LA_E_FREE_PLAN_ACTIVATION_LIMIT_REACHED = 106,
    /// The feature entitlements are invalid.
    LA_E_FEATURE_ENTITLEMENTS_INVALID = 107,
    /// The feature entitlement does not exist.
    LA_E_FEATURE_ENTITLEMENT_NOT_FOUND = 108,
    /// No entitlement set is linked to the license.
    LA_E_ENTITLEMENT_SET_NOT_LINKED = 109,
}

impl From<i32> for LexActivatorStatus {
    fn from(code: i32) -> Self {
        match code {
            0 => LexActivatorStatus::LA_OK,
            1 => LexActivatorStatus::LA_FAIL,
            20 => LexActivatorStatus::LA_EXPIRED,
            21 => LexActivatorStatus::LA_SUSPENDED,
            22 => LexActivatorStatus::LA_GRACE_PERIOD_OVER,
            25 => LexActivatorStatus::LA_TRIAL_EXPIRED,
            26 => LexActivatorStatus::LA_LOCAL_TRIAL_EXPIRED,
            30 => LexActivatorStatus::LA_RELEASE_UPDATE_AVAILABLE,
            31 => LexActivatorStatus::LA_RELEASE_UPDATE_NOT_AVAILABLE,
            32 => LexActivatorStatus::LA_RELEASE_UPDATE_AVAILABLE_NOT_ALLOWED,
            _ => todo!(),
        }  
    }
}

impl From<i32> for LexActivatorError {
    fn from(code: i32) -> Self {
        match code {
            1 => LexActivatorError::LA_FAIL,
            40 => LexActivatorError::LA_E_FILE_PATH,
            41 => LexActivatorError::LA_E_PRODUCT_FILE,
            42 => LexActivatorError::LA_E_PRODUCT_DATA,
            43 => LexActivatorError::LA_E_PRODUCT_ID,
            44 => LexActivatorError::LA_E_SYSTEM_PERMISSION,
            45 => LexActivatorError::LA_E_FILE_PERMISSION,
            46 => LexActivatorError::LA_E_WMIC,
            47 => LexActivatorError::LA_E_TIME,
            48 => LexActivatorError::LA_E_INET,
            49 => LexActivatorError::LA_E_NET_PROXY,
            50 => LexActivatorError::LA_E_HOST_URL,
            51 => LexActivatorError::LA_E_BUFFER_SIZE,
            52 => LexActivatorError::LA_E_APP_VERSION_LENGTH,
            53 => LexActivatorError::LA_E_REVOKED,
            54 => LexActivatorError::LA_E_LICENSE_KEY,
            55 => LexActivatorError::LA_E_LICENSE_TYPE,
            56 => LexActivatorError::LA_E_OFFLINE_RESPONSE_FILE,
            57 => LexActivatorError::LA_E_OFFLINE_RESPONSE_FILE_EXPIRED,
            58 => LexActivatorError::LA_E_ACTIVATION_LIMIT,
            59 => LexActivatorError::LA_E_ACTIVATION_NOT_FOUND,
            60 => LexActivatorError::LA_E_DEACTIVATION_LIMIT,
            61 => LexActivatorError::LA_E_TRIAL_NOT_ALLOWED,
            62 => LexActivatorError::LA_E_TRIAL_ACTIVATION_LIMIT,
            63 => LexActivatorError::LA_E_MACHINE_FINGERPRINT,
            64 => LexActivatorError::LA_E_METADATA_KEY_LENGTH,
            65 => LexActivatorError::LA_E_METADATA_VALUE_LENGTH,
            66 => LexActivatorError::LA_E_ACTIVATION_METADATA_LIMIT,
            67 => LexActivatorError::LA_E_TRIAL_ACTIVATION_METADATA_LIMIT,
            68 => LexActivatorError::LA_E_METADATA_KEY_NOT_FOUND,
            69 => LexActivatorError::LA_E_TIME_MODIFIED,
            70 => LexActivatorError::LA_E_RELEASE_VERSION_FORMAT,
            71 => LexActivatorError::LA_E_AUTHENTICATION_FAILED,
            72 => LexActivatorError::LA_E_METER_ATTRIBUTE_NOT_FOUND,
            73 => LexActivatorError::LA_E_METER_ATTRIBUTE_USES_LIMIT_REACHED,
            74 => LexActivatorError::LA_E_CUSTOM_FINGERPRINT_LENGTH,
            75 => LexActivatorError::LA_E_PRODUCT_VERSION_NOT_LINKED,
            76 => LexActivatorError::LA_E_FEATURE_FLAG_NOT_FOUND,
            77 => LexActivatorError::LA_E_RELEASE_VERSION_NOT_ALLOWED,
            78 => LexActivatorError::LA_E_RELEASE_PLATFORM_LENGTH,
            79 => LexActivatorError::LA_E_RELEASE_CHANNEL_LENGTH,
            80 => LexActivatorError::LA_E_VM,
            81 => LexActivatorError::LA_E_COUNTRY,
            82 => LexActivatorError::LA_E_IP,
            83 => LexActivatorError::LA_E_CONTAINER,
            84 => LexActivatorError::LA_E_RELEASE_VERSION,
            85 => LexActivatorError::LA_E_RELEASE_PLATFORM,
            86 => LexActivatorError::LA_E_RELEASE_CHANNEL,
            87 => LexActivatorError::LA_E_USER_NOT_AUTHENTICATED,
            88 => LexActivatorError::LA_E_TWO_FACTOR_AUTHENTICATION_CODE_MISSING,
            89 => LexActivatorError::LA_E_TWO_FACTOR_AUTHENTICATION_CODE_INVALID,
            90 => LexActivatorError::LA_E_RATE_LIMIT,
            91 => LexActivatorError::LA_E_SERVER,
            92 => LexActivatorError::LA_E_CLIENT,
            93 => LexActivatorError::LA_E_ACCOUNT_ID,
            100 => LexActivatorError::LA_E_LOGIN_TEMPORARILY_LOCKED,
            101 => LexActivatorError::LA_E_AUTHENTICATION_ID_TOKEN_INVALID,
            102 => LexActivatorError::LA_E_OIDC_SSO_NOT_ENABLED,
            103 => LexActivatorError::LA_E_USERS_LIMIT_REACHED,
            104 => LexActivatorError::LA_E_OS_USER,
            105 => LexActivatorError::LA_E_INVALID_PERMISSION_FLAG,
            106 => LexActivatorError::LA_E_FREE_PLAN_ACTIVATION_LIMIT_REACHED,
            107 => LexActivatorError::LA_E_FEATURE_ENTITLEMENTS_INVALID,
            108 => LexActivatorError::LA_E_FEATURE_ENTITLEMENT_NOT_FOUND,
            109 => LexActivatorError::LA_E_ENTITLEMENT_SET_NOT_LINKED,  
            _ => todo!(),
            // Add more mappings as needed
        }
    }
}

impl fmt::Display for LexActivatorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexActivatorStatus::LA_OK => write!(f, "{} Success code.", LexActivatorStatus::LA_OK as i32),
            LexActivatorStatus::LA_FAIL => write!(f, "{} Failure code.", LexActivatorStatus::LA_FAIL as i32),
            LexActivatorStatus::LA_EXPIRED => write!(f, "{} The license has expired or system time has been tampered with. Ensure your date and time settings are correct.", LexActivatorStatus::LA_EXPIRED as i32),
            LexActivatorStatus::LA_SUSPENDED => write!(f, "{} The license has been suspended.", LexActivatorStatus::LA_SUSPENDED as i32),
            LexActivatorStatus::LA_GRACE_PERIOD_OVER => write!(f, "{} The grace period for server sync is over.", LexActivatorStatus::LA_GRACE_PERIOD_OVER as i32),
            LexActivatorStatus::LA_TRIAL_EXPIRED => write!(f, "{} The trial has expired or system time has been tampered with. Ensure your date and time settings are correct.", LexActivatorStatus::LA_TRIAL_EXPIRED as i32),
            LexActivatorStatus::LA_LOCAL_TRIAL_EXPIRED => write!(f, "{} The local trial has expired or system time has been tampered with. Ensure your date and time settings are correct.", LexActivatorStatus::LA_LOCAL_TRIAL_EXPIRED as i32),
            LexActivatorStatus::LA_RELEASE_UPDATE_AVAILABLE => write!(f, "{} A new update is available for the product. This means a new release has been published for the product.", LexActivatorStatus::LA_RELEASE_UPDATE_AVAILABLE as i32),
            LexActivatorStatus::LA_RELEASE_UPDATE_NOT_AVAILABLE => write!(f, "{} No new update is available for the product. The current version is latest.", LexActivatorStatus::LA_RELEASE_UPDATE_NOT_AVAILABLE as i32),
            LexActivatorStatus::LA_RELEASE_UPDATE_AVAILABLE_NOT_ALLOWED => write!(f, "{} The update available is not allowed for this license.", LexActivatorStatus::LA_RELEASE_UPDATE_AVAILABLE_NOT_ALLOWED as i32),
        }
    }
}     

impl fmt::Display for LexActivatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexActivatorError::LA_FAIL => write!(f, "{} Failure code.", LexActivatorError::LA_FAIL as i32),
            LexActivatorError::LA_E_FILE_PATH => write!(f, "{} Invalid file path.", LexActivatorError::LA_E_FILE_PATH as i32),
            LexActivatorError::LA_E_PRODUCT_FILE => write!(f, "{} Invalid or corrupted product file.", LexActivatorError::LA_E_PRODUCT_FILE as i32),
            LexActivatorError::LA_E_PRODUCT_DATA => write!(f, "{} Invalid product data.", LexActivatorError::LA_E_PRODUCT_DATA as i32),
            LexActivatorError::LA_E_PRODUCT_ID => write!(f, "{} The product id is incorrect.", LexActivatorError::LA_E_PRODUCT_ID as i32),
            LexActivatorError::LA_E_SYSTEM_PERMISSION => write!(f, "{} Insufficent system permissions.", LexActivatorError::LA_E_SYSTEM_PERMISSION as i32),
            LexActivatorError::LA_E_FILE_PERMISSION => write!(f, "{} No permission to write to file.", LexActivatorError::LA_E_FILE_PERMISSION as i32),
            LexActivatorError::LA_E_WMIC => write!(f, "{} Fingerprint couldn't be generated because Windows Management Instrumentation (WMI) service has been disabled.", LexActivatorError::LA_E_WMIC as i32),
            LexActivatorError::LA_E_TIME => write!(f, "{} The difference between the network time and the system time is more than allowed clock offset.", LexActivatorError::LA_E_TIME as i32),
            LexActivatorError::LA_E_INET => write!(f, "{} Failed to connect to the server due to network error.", LexActivatorError::LA_E_INET as i32),
            LexActivatorError::LA_E_NET_PROXY => write!(f, "{} Invalid network proxy.", LexActivatorError::LA_E_NET_PROXY as i32),
            LexActivatorError::LA_E_HOST_URL => write!(f, "{} Invalid Cryptlex host url.", LexActivatorError::LA_E_HOST_URL as i32),
            LexActivatorError::LA_E_BUFFER_SIZE => write!(f, "{} The buffer size was smaller than required.", LexActivatorError::LA_E_BUFFER_SIZE as i32),
            LexActivatorError::LA_E_APP_VERSION_LENGTH => write!(f, "{} App version length is more than characters.", LexActivatorError::LA_E_APP_VERSION_LENGTH as i32),
            LexActivatorError::LA_E_REVOKED => write!(f, "{} The license has been revoked.", LexActivatorError::LA_E_REVOKED as i32),
            LexActivatorError::LA_E_LICENSE_KEY => write!(f, "{} Invalid license key.", LexActivatorError::LA_E_LICENSE_KEY as i32),
            LexActivatorError::LA_E_LICENSE_TYPE => write!(f, "{} Invalid license type. Make sure floating license is not being used.", LexActivatorError::LA_E_LICENSE_TYPE as i32),
            LexActivatorError::LA_E_OFFLINE_RESPONSE_FILE => write!(f, "{} Invalid offline activation response file.", LexActivatorError::LA_E_OFFLINE_RESPONSE_FILE as i32),
            LexActivatorError::LA_E_OFFLINE_RESPONSE_FILE_EXPIRED => write!(f, "{} The offline activation response has expired.", LexActivatorError::LA_E_OFFLINE_RESPONSE_FILE_EXPIRED as i32),
            LexActivatorError::LA_E_ACTIVATION_LIMIT => write!(f, "{} The license has reached it's allowed activations limit.", LexActivatorError::LA_E_ACTIVATION_LIMIT as i32),
            LexActivatorError::LA_E_ACTIVATION_NOT_FOUND => write!(f, "{} The license activation was deleted on the server.", LexActivatorError::LA_E_ACTIVATION_NOT_FOUND as i32),
            LexActivatorError::LA_E_DEACTIVATION_LIMIT => write!(f, "{} The license has reached it's allowed deactivations limit.", LexActivatorError::LA_E_DEACTIVATION_LIMIT as i32),
            LexActivatorError::LA_E_TRIAL_NOT_ALLOWED => write!(f, "{} Trial not allowed for the product.", LexActivatorError::LA_E_TRIAL_NOT_ALLOWED as i32),
            LexActivatorError::LA_E_TRIAL_ACTIVATION_LIMIT => write!(f, "{} Your account has reached it's trial activations limit and trial not allowed for the product.", LexActivatorError::LA_E_TRIAL_ACTIVATION_LIMIT as i32),
            LexActivatorError::LA_E_MACHINE_FINGERPRINT => write!(f, "{} Machine fingerprint has changed since activation.", LexActivatorError::LA_E_MACHINE_FINGERPRINT as i32),
            LexActivatorError::LA_E_METADATA_KEY_LENGTH => write!(f, "{} Metadata key length is more than 256 characters.", LexActivatorError::LA_E_METADATA_KEY_LENGTH as i32),
            LexActivatorError::LA_E_METADATA_VALUE_LENGTH => write!(f, "{} Metadata value length is more than 256 characters.", LexActivatorError::LA_E_METADATA_VALUE_LENGTH as i32),
            LexActivatorError::LA_E_ACTIVATION_METADATA_LIMIT => write!(f, "{} The license has reached it's metadata fields limit.", LexActivatorError::LA_E_ACTIVATION_METADATA_LIMIT as i32),
            LexActivatorError::LA_E_TRIAL_ACTIVATION_METADATA_LIMIT => write!(f, "{} The trial has reached it's metadata fields limit.", LexActivatorError::LA_E_TRIAL_ACTIVATION_METADATA_LIMIT as i32),
            LexActivatorError::LA_E_METADATA_KEY_NOT_FOUND => write!(f, "{} The metadata key does not exist.", LexActivatorError::LA_E_METADATA_KEY_NOT_FOUND as i32),
            LexActivatorError::LA_E_TIME_MODIFIED => write!(f, "{} The system time has been tampered (backdated).", LexActivatorError::LA_E_TIME_MODIFIED as i32),
            LexActivatorError::LA_E_RELEASE_VERSION_FORMAT => write!(f, "{} Invalid version format.", LexActivatorError::LA_E_RELEASE_VERSION_FORMAT as i32),
            LexActivatorError::LA_E_AUTHENTICATION_FAILED => write!(f, "{} Incorrect email or password.", LexActivatorError::LA_E_AUTHENTICATION_FAILED as i32),
            LexActivatorError::LA_E_METER_ATTRIBUTE_NOT_FOUND => write!(f, "{} The meter attribute does not exist.", LexActivatorError::LA_E_METER_ATTRIBUTE_NOT_FOUND as i32),
            LexActivatorError::LA_E_METER_ATTRIBUTE_USES_LIMIT_REACHED => write!(f, "{} The meter attribute has reached it's usage limit.", LexActivatorError::LA_E_METER_ATTRIBUTE_USES_LIMIT_REACHED as i32),
            LexActivatorError::LA_E_CUSTOM_FINGERPRINT_LENGTH => write!(f, "{} Custom device fingerprint length is less than 64 characters or more than 256 characters.", LexActivatorError::LA_E_CUSTOM_FINGERPRINT_LENGTH as i32),
            LexActivatorError::LA_E_PRODUCT_VERSION_NOT_LINKED => write!(f, "{} No product version is linked with the license.", LexActivatorError::LA_E_PRODUCT_VERSION_NOT_LINKED as i32),
            LexActivatorError::LA_E_FEATURE_FLAG_NOT_FOUND => write!(f, "{} The product version feature flag does not exist.", LexActivatorError::LA_E_FEATURE_FLAG_NOT_FOUND as i32),
            LexActivatorError::LA_E_RELEASE_VERSION_NOT_ALLOWED => write!(f, "{} The release version is not allowed.", LexActivatorError::LA_E_RELEASE_VERSION_NOT_ALLOWED as i32),
            LexActivatorError::LA_E_RELEASE_PLATFORM_LENGTH => write!(f, "{} Release platform length is more than 256 characters.", LexActivatorError::LA_E_RELEASE_PLATFORM_LENGTH as i32),
            LexActivatorError::LA_E_RELEASE_CHANNEL_LENGTH => write!(f, "{} Release channel length is more than 256 characters.", LexActivatorError::LA_E_RELEASE_CHANNEL_LENGTH as i32),
            LexActivatorError::LA_E_VM => write!(f, "{} Application is running inside virtual machine / hypervisor and activation has been disallowed in the VM.", LexActivatorError::LA_E_VM as i32),
            LexActivatorError::LA_E_COUNTRY => write!(f, "{} Country is not allowed.", LexActivatorError::LA_E_COUNTRY as i32),
            LexActivatorError::LA_E_IP => write!(f, "{} IP address is not allowed.", LexActivatorError::LA_E_IP as i32),
            LexActivatorError::LA_E_CONTAINER => write!(f, "{} Application is being run inside a container and activation has been disallowed in the container.", LexActivatorError::LA_E_CONTAINER as i32),
            LexActivatorError::LA_E_RELEASE_VERSION => write!(f, "{} Invalid release version. Make sure the release version uses the following formats: x.x, x.x.x, x.x.x.x (where x is a number).", LexActivatorError::LA_E_RELEASE_VERSION as i32),
            LexActivatorError::LA_E_RELEASE_PLATFORM => write!(f, "{} Release platform not set.", LexActivatorError::LA_E_RELEASE_PLATFORM as i32),
            LexActivatorError::LA_E_RELEASE_CHANNEL => write!(f, "{} Release channel not set.", LexActivatorError::LA_E_RELEASE_CHANNEL as i32),
            LexActivatorError::LA_E_USER_NOT_AUTHENTICATED => write!(f, "{} The user is not authenticated.", LexActivatorError::LA_E_USER_NOT_AUTHENTICATED as i32),
            LexActivatorError::LA_E_TWO_FACTOR_AUTHENTICATION_CODE_MISSING => write!(f, "{} The two-factor authentication code for the user authentication is missing.", LexActivatorError::LA_E_TWO_FACTOR_AUTHENTICATION_CODE_MISSING as i32),
            LexActivatorError::LA_E_TWO_FACTOR_AUTHENTICATION_CODE_INVALID => write!(f, "{} he two-factor authentication code provided by the user is invalid.", LexActivatorError::LA_E_TWO_FACTOR_AUTHENTICATION_CODE_INVALID as i32),
            LexActivatorError::LA_E_RATE_LIMIT => write!(f, "{} Rate limit for API has reached, try again later.", LexActivatorError::LA_E_RATE_LIMIT as i32),
            LexActivatorError::LA_E_SERVER => write!(f, "{} Server error.", LexActivatorError::LA_E_SERVER as i32),
            LexActivatorError::LA_E_CLIENT => write!(f, "{} Client error.", LexActivatorError::LA_E_CLIENT as i32),
            LexActivatorError::LA_E_LOGIN_TEMPORARILY_LOCKED => write!(f, "{} The user account has been temporarily locked for 5 mins due to 5 failed attempts.", LexActivatorError::LA_E_LOGIN_TEMPORARILY_LOCKED as i32),
            LexActivatorError::LA_E_AUTHENTICATION_ID_TOKEN_INVALID => write!(f, "{} Invalid authentication ID token.", LexActivatorError::LA_E_AUTHENTICATION_ID_TOKEN_INVALID as i32),
            LexActivatorError::LA_E_OIDC_SSO_NOT_ENABLED => write!(f, "{} OIDC SSO is not enabled.", LexActivatorError::LA_E_OIDC_SSO_NOT_ENABLED as i32),
            LexActivatorError::LA_E_USERS_LIMIT_REACHED => write!(f, "{} The allowed users for this account has reached its limit.", LexActivatorError::LA_E_USERS_LIMIT_REACHED as i32),
            LexActivatorError::LA_E_OS_USER => write!(f, "{} OS user has changed since activation and the license is user-locked.", LexActivatorError::LA_E_OS_USER as i32),
            LexActivatorError::LA_E_INVALID_PERMISSION_FLAG => write!(f, "{} Invalid permission flag.", LexActivatorError::LA_E_INVALID_PERMISSION_FLAG as i32),
            LexActivatorError::LA_E_FREE_PLAN_ACTIVATION_LIMIT_REACHED => write!(f, "{} The free plan has reached its activation limit.", LexActivatorError::LA_E_FREE_PLAN_ACTIVATION_LIMIT_REACHED as i32),
            LexActivatorError::LA_E_ACCOUNT_ID => write!(f, "{} Invalid account ID.", LexActivatorError::LA_E_ACCOUNT_ID as i32),
            LexActivatorError::LA_E_FEATURE_ENTITLEMENTS_INVALID => write!(f, "{} Invalid feature entitlements.", LexActivatorError::LA_E_FEATURE_ENTITLEMENTS_INVALID as i32),
            LexActivatorError::LA_E_FEATURE_ENTITLEMENT_NOT_FOUND => write!(f, "{} The feature entitlement does not exist.", LexActivatorError::LA_E_FEATURE_ENTITLEMENT_NOT_FOUND as i32),
            LexActivatorError::LA_E_ENTITLEMENT_SET_NOT_LINKED => write!(f, "{} No entitlement set is linked to the license.", LexActivatorError::LA_E_ENTITLEMENT_SET_NOT_LINKED as i32),
        }
    }
}

impl From<NulError> for LexActivatorError {
    fn from(_: NulError) -> Self {
        LexActivatorError::LA_E_CLIENT  
    }
}

#[derive(Debug)]
#[repr(i32)]
pub enum LexActivatorCode {
    Status(LexActivatorStatus),
    Error(LexActivatorError),
}

impl LexActivatorCode {
    pub fn from_i32(code: i32) -> Self {
        match code {
            0..=32 => LexActivatorCode::Status(LexActivatorStatus::from(code)),
            40..=106 => LexActivatorCode::Error(LexActivatorError::from(code)),
            _ => LexActivatorCode::Error(LexActivatorError::LA_E_CLIENT), // Fallback to a general error
        }
    }
}