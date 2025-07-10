use serde::{Deserialize, Serialize};

/// TransactionSubStatus : See [Transaction substatuses](https://developers.fireblocks.com/reference/transaction-substatuses) for the list of transaction sub statuses
/// See [Transaction substatuses](https://developers.fireblocks.com/reference/transaction-substatuses) for the list of transaction sub statuses
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionSubStatus {
    #[serde(rename = "3RD_PARTY_PROCESSING")]
    Variant3RdPartyProcessing,
    #[serde(rename = "3RD_PARTY_PENDING_SERVICE_MANUAL_APPROVAL")]
    Variant3RdPartyPendingServiceManualApproval,
    #[serde(rename = "PENDING_3RD_PARTY_MANUAL_APPROVAL")]
    Pending3RdPartyManualApproval,
    #[serde(rename = "3RD_PARTY_CONFIRMING")]
    Variant3RdPartyConfirming,
    #[serde(rename = "PENDING_BLOCKCHAIN_CONFIRMATIONS")]
    PendingBlockchainConfirmations,
    #[serde(rename = "3RD_PARTY_COMPLETED")]
    Variant3RdPartyCompleted,
    #[serde(rename = "COMPLETED_BUT_3RD_PARTY_FAILED")]
    CompletedBut3RdPartyFailed,
    #[serde(rename = "COMPLETED_BUT_3RD_PARTY_REJECTED")]
    CompletedBut3RdPartyRejected,
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    #[serde(rename = "BLOCKED_BY_POLICY")]
    BlockedByPolicy,
    #[serde(rename = "3RD_PARTY_CANCELLED")]
    Variant3RdPartyCancelled,
    #[serde(rename = "3RD_PARTY_REJECTED")]
    Variant3RdPartyRejected,
    #[serde(rename = "CANCELLED_BY_USER")]
    CancelledByUser,
    #[serde(rename = "CANCELLED_BY_USER_REQUEST")]
    CancelledByUserRequest,
    #[serde(rename = "REJECTED_BY_USER")]
    RejectedByUser,
    #[serde(rename = "AUTO_FREEZE")]
    AutoFreeze,
    #[serde(rename = "FROZEN_MANUALLY")]
    FrozenManually,
    #[serde(rename = "REJECTED_AML_SCREENING")]
    RejectedAmlScreening,
    #[serde(rename = "ACTUAL_FEE_TOO_HIGH")]
    ActualFeeTooHigh,
    #[serde(rename = "ADDRESS_WHITELISTING_SUSPENDED")]
    AddressWhitelistingSuspended,
    #[serde(rename = "AMOUNT_TOO_SMALL")]
    AmountTooSmall,
    #[serde(rename = "AUTHORIZATION_FAILED")]
    AuthorizationFailed,
    #[serde(rename = "AUTHORIZER_NOT_FOUND")]
    AuthorizerNotFound,
    #[serde(rename = "ENV_UNSUPPORTED_ASSET")]
    EnvUnsupportedAsset,
    #[serde(rename = "ERROR_UNSUPPORTED_TRANSACTION_TYPE")]
    ErrorUnsupportedTransactionType,
    #[serde(rename = "FAIL_ON_LOW_FEE")]
    FailOnLowFee,
    #[serde(rename = "GAS_LIMIT_TOO_LOW")]
    GasLimitTooLow,
    #[serde(rename = "GAS_PRICE_TOO_LOW_FOR_RBF")]
    GasPriceTooLowForRbf,
    #[serde(rename = "INCOMPLETE_USER_SETUP")]
    IncompleteUserSetup,
    #[serde(rename = "INSUFFICIENT_FUNDS")]
    InsufficientFunds,
    #[serde(rename = "INSUFFICIENT_FUNDS_FOR_FEE")]
    InsufficientFundsForFee,
    #[serde(rename = "INTEGRATION_SUSPENDED")]
    IntegrationSuspended,
    #[serde(rename = "INVALID_ADDRESS")]
    InvalidAddress,
    #[serde(rename = "INVALID_CONTRACT_CALL_DATA")]
    InvalidContractCallData,
    #[serde(rename = "INVALID_FEE_PARAMS")]
    InvalidFeeParams,
    #[serde(rename = "INVALID_NONCE_FOR_RBF")]
    InvalidNonceForRbf,
    #[serde(rename = "INVALID_TAG_OR_MEMO")]
    InvalidTagOrMemo,
    #[serde(rename = "INVALID_UNMANAGED_WALLET")]
    InvalidUnmanagedWallet,
    #[serde(rename = "MAX_FEE_EXCEEDED")]
    MaxFeeExceeded,
    #[serde(rename = "MISSING_TAG_OR_MEMO")]
    MissingTagOrMemo,
    #[serde(rename = "NEED_MORE_TO_CREATE_DESTINATION")]
    NeedMoreToCreateDestination,
    #[serde(rename = "NO_MORE_PREPROCESSED_INDEXES")]
    NoMorePreprocessedIndexes,
    #[serde(rename = "NON_EXISTING_ACCOUNT_NAME")]
    NonExistingAccountName,
    #[serde(rename = "RAW_MSG_EMPTY_OR_INVALID")]
    RawMsgEmptyOrInvalid,
    #[serde(rename = "RAW_MSG_LEN_INVALID")]
    RawMsgLenInvalid,
    #[serde(rename = "TOO_MANY_INPUTS")]
    TooManyInputs,
    #[serde(rename = "TX_SIZE_EXCEEDED_MAX")]
    TxSizeExceededMax,
    #[serde(rename = "UNAUTHORISED_DEVICE")]
    UnauthorisedDevice,
    #[serde(rename = "UNAUTHORISED_USER")]
    UnauthorisedUser,
    #[serde(rename = "UNALLOWED_RAW_PARAM_COMBINATION")]
    UnallowedRawParamCombination,
    #[serde(rename = "UNSUPPORTED_OPERATION")]
    UnsupportedOperation,
    #[serde(rename = "UNSUPPORTED_TRANSACTION_TYPE")]
    UnsupportedTransactionType,
    #[serde(rename = "ZERO_BALANCE_IN_PERMANENT_ADDRESS")]
    ZeroBalanceInPermanentAddress,
    #[serde(rename = "OUT_OF_DATE_SIGNING_KEYS")]
    OutOfDateSigningKeys,
    #[serde(rename = "CONNECTIVITY_ERROR")]
    ConnectivityError,
    #[serde(rename = "ERROR_ASYNC_TX_IN_FLIGHT")]
    ErrorAsyncTxInFlight,
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError,
    #[serde(rename = "INVALID_NONCE_TOO_HIGH")]
    InvalidNonceTooHigh,
    #[serde(rename = "INVALID_NONCE_TOO_LOW")]
    InvalidNonceTooLow,
    #[serde(rename = "INVALID_ROUTING_DESTINATION")]
    InvalidRoutingDestination,
    #[serde(rename = "LOCKING_NONCE_ACCOUNT_TIMEOUT")]
    LockingNonceAccountTimeout,
    #[serde(rename = "NETWORK_ROUTING_MISMATCH")]
    NetworkRoutingMismatch,
    #[serde(rename = "NONCE_ALLOCATION_FAILED")]
    NonceAllocationFailed,
    #[serde(rename = "RESOURCE_ALREADY_EXISTS")]
    ResourceAlreadyExists,
    #[serde(rename = "SIGNER_NOT_FOUND")]
    SignerNotFound,
    #[serde(rename = "SIGNING_ERROR")]
    SigningError,
    #[serde(rename = "TIMEOUT")]
    Timeout,
    #[serde(rename = "TX_OUTDATED")]
    TxOutdated,
    #[serde(rename = "UNKNOWN_ERROR")]
    UnknownError,
    #[serde(rename = "VAULT_WALLET_NOT_READY")]
    VaultWalletNotReady,
    #[serde(rename = "UNSUPPORTED_MEDIA_TYPE")]
    UnsupportedMediaType,
    #[serde(rename = "ADDRESS_NOT_WHITELISTED")]
    AddressNotWhitelisted,
    #[serde(rename = "API_KEY_MISMATCH")]
    ApiKeyMismatch,
    #[serde(rename = "ASSET_NOT_ENABLED_ON_DESTINATION")]
    AssetNotEnabledOnDestination,
    #[serde(rename = "DEST_TYPE_NOT_SUPPORTED")]
    DestTypeNotSupported,
    #[serde(rename = "EXCEEDED_DECIMAL_PRECISION")]
    ExceededDecimalPrecision,
    #[serde(rename = "EXCHANGE_CONFIGURATION_MISMATCH")]
    ExchangeConfigurationMismatch,
    #[serde(rename = "EXCHANGE_VERSION_INCOMPATIBLE")]
    ExchangeVersionIncompatible,
    #[serde(rename = "INVALID_EXCHANGE_ACCOUNT")]
    InvalidExchangeAccount,
    #[serde(rename = "METHOD_NOT_ALLOWED")]
    MethodNotAllowed,
    #[serde(rename = "NON_EXISTENT_AUTO_ACCOUNT")]
    NonExistentAutoAccount,
    #[serde(rename = "ON_PREMISE_CONNECTIVITY_ERROR")]
    OnPremiseConnectivityError,
    #[serde(rename = "PEER_ACCOUNT_DOES_NOT_EXIST")]
    PeerAccountDoesNotExist,
    #[serde(rename = "THIRD_PARTY_MISSING_ACCOUNT")]
    ThirdPartyMissingAccount,
    #[serde(rename = "UNAUTHORISED_IP_WHITELISTING")]
    UnauthorisedIpWhitelisting,
    #[serde(rename = "UNAUTHORISED_MISSING_CREDENTIALS")]
    UnauthorisedMissingCredentials,
    #[serde(rename = "UNAUTHORISED_MISSING_PERMISSION")]
    UnauthorisedMissingPermission,
    #[serde(rename = "UNAUTHORISED_OTP_FAILED")]
    UnauthorisedOtpFailed,
    #[serde(rename = "WITHDRAW_LIMIT")]
    WithdrawLimit,
    #[serde(rename = "3RD_PARTY_FAILED")]
    Variant3RdPartyFailed,
    #[serde(rename = "API_CALL_LIMIT")]
    ApiCallLimit,
    #[serde(rename = "API_INVALID_SIGNATURE")]
    ApiInvalidSignature,
    #[serde(rename = "CANCELLED_EXTERNALLY")]
    CancelledExternally,
    #[serde(rename = "FAILED_AML_SCREENING")]
    FailedAmlScreening,
    #[serde(rename = "INVALID_FEE")]
    InvalidFee,
    #[serde(rename = "INVALID_THIRD_PARTY_RESPONSE")]
    InvalidThirdPartyResponse,
    #[serde(rename = "MANUAL_DEPOSIT_ADDRESS_REQUIRED")]
    ManualDepositAddressRequired,
    #[serde(rename = "MISSING_DEPOSIT_ADDRESS")]
    MissingDepositAddress,
    #[serde(rename = "NO_DEPOSIT_ADDRESS")]
    NoDepositAddress,
    #[serde(rename = "SUB_ACCOUNTS_NOT_SUPPORTED")]
    SubAccountsNotSupported,
    #[serde(rename = "SPEND_COINBASE_TOO_EARLY")]
    SpendCoinbaseTooEarly,
    #[serde(rename = "THIRD_PARTY_INTERNAL_ERROR")]
    ThirdPartyInternalError,
    #[serde(rename = "TX_ID_NOT_ACCEPTED_BY_THIRD_PARTY")]
    TxIdNotAcceptedByThirdParty,
    #[serde(rename = "UNSUPPORTED_ASSET")]
    UnsupportedAsset,
    #[serde(rename = "DOUBLE_SPENDING")]
    DoubleSpending,
    #[serde(rename = "DROPPED_BY_BLOCKCHAIN")]
    DroppedByBlockchain,
    #[serde(rename = "INSUFFICIENT_RESERVED_FUNDING")]
    InsufficientReservedFunding,
    #[serde(rename = "INVALID_SIGNATURE")]
    InvalidSignature,
    #[serde(rename = "PARTIALLY_FAILED")]
    PartiallyFailed,
    #[serde(rename = "POWERUP_SUGGESTION_FAILURE")]
    PowerupSuggestionFailure,
    #[serde(rename = "REACHED_MEMPOOL_LIMIT_FOR_ACCOUNT")]
    ReachedMempoolLimitForAccount,
    #[serde(rename = "REJECTED_BY_BLOCKCHAIN")]
    RejectedByBlockchain,
    #[serde(rename = "SMART_CONTRACT_EXECUTION_FAILED")]
    SmartContractExecutionFailed,
    #[serde(rename = "TOO_LONG_MEMPOOL_CHAIN")]
    TooLongMempoolChain,
    #[serde(rename = "")]
    Empty,
}

impl std::fmt::Display for TransactionSubStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant3RdPartyProcessing => write!(f, "3RD_PARTY_PROCESSING"),
            Self::Variant3RdPartyPendingServiceManualApproval => {
                write!(f, "3RD_PARTY_PENDING_SERVICE_MANUAL_APPROVAL")
            }
            Self::Pending3RdPartyManualApproval => write!(f, "PENDING_3RD_PARTY_MANUAL_APPROVAL"),
            Self::Variant3RdPartyConfirming => write!(f, "3RD_PARTY_CONFIRMING"),
            Self::PendingBlockchainConfirmations => write!(f, "PENDING_BLOCKCHAIN_CONFIRMATIONS"),
            Self::Variant3RdPartyCompleted => write!(f, "3RD_PARTY_COMPLETED"),
            Self::CompletedBut3RdPartyFailed => write!(f, "COMPLETED_BUT_3RD_PARTY_FAILED"),
            Self::CompletedBut3RdPartyRejected => write!(f, "COMPLETED_BUT_3RD_PARTY_REJECTED"),
            Self::Confirmed => write!(f, "CONFIRMED"),
            Self::BlockedByPolicy => write!(f, "BLOCKED_BY_POLICY"),
            Self::Variant3RdPartyCancelled => write!(f, "3RD_PARTY_CANCELLED"),
            Self::Variant3RdPartyRejected => write!(f, "3RD_PARTY_REJECTED"),
            Self::CancelledByUser => write!(f, "CANCELLED_BY_USER"),
            Self::CancelledByUserRequest => write!(f, "CANCELLED_BY_USER_REQUEST"),
            Self::RejectedByUser => write!(f, "REJECTED_BY_USER"),
            Self::AutoFreeze => write!(f, "AUTO_FREEZE"),
            Self::FrozenManually => write!(f, "FROZEN_MANUALLY"),
            Self::RejectedAmlScreening => write!(f, "REJECTED_AML_SCREENING"),
            Self::ActualFeeTooHigh => write!(f, "ACTUAL_FEE_TOO_HIGH"),
            Self::AddressWhitelistingSuspended => write!(f, "ADDRESS_WHITELISTING_SUSPENDED"),
            Self::AmountTooSmall => write!(f, "AMOUNT_TOO_SMALL"),
            Self::AuthorizationFailed => write!(f, "AUTHORIZATION_FAILED"),
            Self::AuthorizerNotFound => write!(f, "AUTHORIZER_NOT_FOUND"),
            Self::EnvUnsupportedAsset => write!(f, "ENV_UNSUPPORTED_ASSET"),
            Self::ErrorUnsupportedTransactionType => {
                write!(f, "ERROR_UNSUPPORTED_TRANSACTION_TYPE")
            }
            Self::FailOnLowFee => write!(f, "FAIL_ON_LOW_FEE"),
            Self::GasLimitTooLow => write!(f, "GAS_LIMIT_TOO_LOW"),
            Self::GasPriceTooLowForRbf => write!(f, "GAS_PRICE_TOO_LOW_FOR_RBF"),
            Self::IncompleteUserSetup => write!(f, "INCOMPLETE_USER_SETUP"),
            Self::InsufficientFunds => write!(f, "INSUFFICIENT_FUNDS"),
            Self::InsufficientFundsForFee => write!(f, "INSUFFICIENT_FUNDS_FOR_FEE"),
            Self::IntegrationSuspended => write!(f, "INTEGRATION_SUSPENDED"),
            Self::InvalidAddress => write!(f, "INVALID_ADDRESS"),
            Self::InvalidContractCallData => write!(f, "INVALID_CONTRACT_CALL_DATA"),
            Self::InvalidFeeParams => write!(f, "INVALID_FEE_PARAMS"),
            Self::InvalidNonceForRbf => write!(f, "INVALID_NONCE_FOR_RBF"),
            Self::InvalidTagOrMemo => write!(f, "INVALID_TAG_OR_MEMO"),
            Self::InvalidUnmanagedWallet => write!(f, "INVALID_UNMANAGED_WALLET"),
            Self::MaxFeeExceeded => write!(f, "MAX_FEE_EXCEEDED"),
            Self::MissingTagOrMemo => write!(f, "MISSING_TAG_OR_MEMO"),
            Self::NeedMoreToCreateDestination => write!(f, "NEED_MORE_TO_CREATE_DESTINATION"),
            Self::NoMorePreprocessedIndexes => write!(f, "NO_MORE_PREPROCESSED_INDEXES"),
            Self::NonExistingAccountName => write!(f, "NON_EXISTING_ACCOUNT_NAME"),
            Self::RawMsgEmptyOrInvalid => write!(f, "RAW_MSG_EMPTY_OR_INVALID"),
            Self::RawMsgLenInvalid => write!(f, "RAW_MSG_LEN_INVALID"),
            Self::TooManyInputs => write!(f, "TOO_MANY_INPUTS"),
            Self::TxSizeExceededMax => write!(f, "TX_SIZE_EXCEEDED_MAX"),
            Self::UnauthorisedDevice => write!(f, "UNAUTHORISED_DEVICE"),
            Self::UnauthorisedUser => write!(f, "UNAUTHORISED_USER"),
            Self::UnallowedRawParamCombination => write!(f, "UNALLOWED_RAW_PARAM_COMBINATION"),
            Self::UnsupportedOperation => write!(f, "UNSUPPORTED_OPERATION"),
            Self::UnsupportedTransactionType => write!(f, "UNSUPPORTED_TRANSACTION_TYPE"),
            Self::ZeroBalanceInPermanentAddress => write!(f, "ZERO_BALANCE_IN_PERMANENT_ADDRESS"),
            Self::OutOfDateSigningKeys => write!(f, "OUT_OF_DATE_SIGNING_KEYS"),
            Self::ConnectivityError => write!(f, "CONNECTIVITY_ERROR"),
            Self::ErrorAsyncTxInFlight => write!(f, "ERROR_ASYNC_TX_IN_FLIGHT"),
            Self::InternalError => write!(f, "INTERNAL_ERROR"),
            Self::InvalidNonceTooHigh => write!(f, "INVALID_NONCE_TOO_HIGH"),
            Self::InvalidNonceTooLow => write!(f, "INVALID_NONCE_TOO_LOW"),
            Self::InvalidRoutingDestination => write!(f, "INVALID_ROUTING_DESTINATION"),
            Self::LockingNonceAccountTimeout => write!(f, "LOCKING_NONCE_ACCOUNT_TIMEOUT"),
            Self::NetworkRoutingMismatch => write!(f, "NETWORK_ROUTING_MISMATCH"),
            Self::NonceAllocationFailed => write!(f, "NONCE_ALLOCATION_FAILED"),
            Self::ResourceAlreadyExists => write!(f, "RESOURCE_ALREADY_EXISTS"),
            Self::SignerNotFound => write!(f, "SIGNER_NOT_FOUND"),
            Self::SigningError => write!(f, "SIGNING_ERROR"),
            Self::Timeout => write!(f, "TIMEOUT"),
            Self::TxOutdated => write!(f, "TX_OUTDATED"),
            Self::UnknownError => write!(f, "UNKNOWN_ERROR"),
            Self::VaultWalletNotReady => write!(f, "VAULT_WALLET_NOT_READY"),
            Self::UnsupportedMediaType => write!(f, "UNSUPPORTED_MEDIA_TYPE"),
            Self::AddressNotWhitelisted => write!(f, "ADDRESS_NOT_WHITELISTED"),
            Self::ApiKeyMismatch => write!(f, "API_KEY_MISMATCH"),
            Self::AssetNotEnabledOnDestination => write!(f, "ASSET_NOT_ENABLED_ON_DESTINATION"),
            Self::DestTypeNotSupported => write!(f, "DEST_TYPE_NOT_SUPPORTED"),
            Self::ExceededDecimalPrecision => write!(f, "EXCEEDED_DECIMAL_PRECISION"),
            Self::ExchangeConfigurationMismatch => write!(f, "EXCHANGE_CONFIGURATION_MISMATCH"),
            Self::ExchangeVersionIncompatible => write!(f, "EXCHANGE_VERSION_INCOMPATIBLE"),
            Self::InvalidExchangeAccount => write!(f, "INVALID_EXCHANGE_ACCOUNT"),
            Self::MethodNotAllowed => write!(f, "METHOD_NOT_ALLOWED"),
            Self::NonExistentAutoAccount => write!(f, "NON_EXISTENT_AUTO_ACCOUNT"),
            Self::OnPremiseConnectivityError => write!(f, "ON_PREMISE_CONNECTIVITY_ERROR"),
            Self::PeerAccountDoesNotExist => write!(f, "PEER_ACCOUNT_DOES_NOT_EXIST"),
            Self::ThirdPartyMissingAccount => write!(f, "THIRD_PARTY_MISSING_ACCOUNT"),
            Self::UnauthorisedIpWhitelisting => write!(f, "UNAUTHORISED_IP_WHITELISTING"),
            Self::UnauthorisedMissingCredentials => write!(f, "UNAUTHORISED_MISSING_CREDENTIALS"),
            Self::UnauthorisedMissingPermission => write!(f, "UNAUTHORISED_MISSING_PERMISSION"),
            Self::UnauthorisedOtpFailed => write!(f, "UNAUTHORISED_OTP_FAILED"),
            Self::WithdrawLimit => write!(f, "WITHDRAW_LIMIT"),
            Self::Variant3RdPartyFailed => write!(f, "3RD_PARTY_FAILED"),
            Self::ApiCallLimit => write!(f, "API_CALL_LIMIT"),
            Self::ApiInvalidSignature => write!(f, "API_INVALID_SIGNATURE"),
            Self::CancelledExternally => write!(f, "CANCELLED_EXTERNALLY"),
            Self::FailedAmlScreening => write!(f, "FAILED_AML_SCREENING"),
            Self::InvalidFee => write!(f, "INVALID_FEE"),
            Self::InvalidThirdPartyResponse => write!(f, "INVALID_THIRD_PARTY_RESPONSE"),
            Self::ManualDepositAddressRequired => write!(f, "MANUAL_DEPOSIT_ADDRESS_REQUIRED"),
            Self::MissingDepositAddress => write!(f, "MISSING_DEPOSIT_ADDRESS"),
            Self::NoDepositAddress => write!(f, "NO_DEPOSIT_ADDRESS"),
            Self::SubAccountsNotSupported => write!(f, "SUB_ACCOUNTS_NOT_SUPPORTED"),
            Self::SpendCoinbaseTooEarly => write!(f, "SPEND_COINBASE_TOO_EARLY"),
            Self::ThirdPartyInternalError => write!(f, "THIRD_PARTY_INTERNAL_ERROR"),
            Self::TxIdNotAcceptedByThirdParty => write!(f, "TX_ID_NOT_ACCEPTED_BY_THIRD_PARTY"),
            Self::UnsupportedAsset => write!(f, "UNSUPPORTED_ASSET"),
            Self::DoubleSpending => write!(f, "DOUBLE_SPENDING"),
            Self::DroppedByBlockchain => write!(f, "DROPPED_BY_BLOCKCHAIN"),
            Self::InsufficientReservedFunding => write!(f, "INSUFFICIENT_RESERVED_FUNDING"),
            Self::InvalidSignature => write!(f, "INVALID_SIGNATURE"),
            Self::PartiallyFailed => write!(f, "PARTIALLY_FAILED"),
            Self::PowerupSuggestionFailure => write!(f, "POWERUP_SUGGESTION_FAILURE"),
            Self::ReachedMempoolLimitForAccount => write!(f, "REACHED_MEMPOOL_LIMIT_FOR_ACCOUNT"),
            Self::RejectedByBlockchain => write!(f, "REJECTED_BY_BLOCKCHAIN"),
            Self::SmartContractExecutionFailed => write!(f, "SMART_CONTRACT_EXECUTION_FAILED"),
            Self::TooLongMempoolChain => write!(f, "TOO_LONG_MEMPOOL_CHAIN"),
            Self::Empty => write!(f, ""),
        }
    }
}

impl Default for TransactionSubStatus {
    fn default() -> TransactionSubStatus {
        Self::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_status_display() {
        let test_cases = [
            (
                TransactionSubStatus::Variant3RdPartyProcessing,
                "3RD_PARTY_PROCESSING",
            ),
            (
                TransactionSubStatus::Variant3RdPartyPendingServiceManualApproval,
                "3RD_PARTY_PENDING_SERVICE_MANUAL_APPROVAL",
            ),
            (
                TransactionSubStatus::Pending3RdPartyManualApproval,
                "PENDING_3RD_PARTY_MANUAL_APPROVAL",
            ),
            (
                TransactionSubStatus::Variant3RdPartyConfirming,
                "3RD_PARTY_CONFIRMING",
            ),
            (
                TransactionSubStatus::PendingBlockchainConfirmations,
                "PENDING_BLOCKCHAIN_CONFIRMATIONS",
            ),
            (
                TransactionSubStatus::Variant3RdPartyCompleted,
                "3RD_PARTY_COMPLETED",
            ),
            (
                TransactionSubStatus::CompletedBut3RdPartyFailed,
                "COMPLETED_BUT_3RD_PARTY_FAILED",
            ),
            (
                TransactionSubStatus::CompletedBut3RdPartyRejected,
                "COMPLETED_BUT_3RD_PARTY_REJECTED",
            ),
            (TransactionSubStatus::Confirmed, "CONFIRMED"),
            (TransactionSubStatus::BlockedByPolicy, "BLOCKED_BY_POLICY"),
            (
                TransactionSubStatus::Variant3RdPartyCancelled,
                "3RD_PARTY_CANCELLED",
            ),
            (
                TransactionSubStatus::Variant3RdPartyRejected,
                "3RD_PARTY_REJECTED",
            ),
            (TransactionSubStatus::CancelledByUser, "CANCELLED_BY_USER"),
            (
                TransactionSubStatus::CancelledByUserRequest,
                "CANCELLED_BY_USER_REQUEST",
            ),
            (TransactionSubStatus::RejectedByUser, "REJECTED_BY_USER"),
            (TransactionSubStatus::AutoFreeze, "AUTO_FREEZE"),
            (TransactionSubStatus::FrozenManually, "FROZEN_MANUALLY"),
            (
                TransactionSubStatus::RejectedAmlScreening,
                "REJECTED_AML_SCREENING",
            ),
            (
                TransactionSubStatus::ActualFeeTooHigh,
                "ACTUAL_FEE_TOO_HIGH",
            ),
            (
                TransactionSubStatus::AddressWhitelistingSuspended,
                "ADDRESS_WHITELISTING_SUSPENDED",
            ),
            (TransactionSubStatus::AmountTooSmall, "AMOUNT_TOO_SMALL"),
            (
                TransactionSubStatus::AuthorizationFailed,
                "AUTHORIZATION_FAILED",
            ),
            (
                TransactionSubStatus::AuthorizerNotFound,
                "AUTHORIZER_NOT_FOUND",
            ),
            (
                TransactionSubStatus::EnvUnsupportedAsset,
                "ENV_UNSUPPORTED_ASSET",
            ),
            (
                TransactionSubStatus::ErrorUnsupportedTransactionType,
                "ERROR_UNSUPPORTED_TRANSACTION_TYPE",
            ),
            (TransactionSubStatus::FailOnLowFee, "FAIL_ON_LOW_FEE"),
            (TransactionSubStatus::GasLimitTooLow, "GAS_LIMIT_TOO_LOW"),
            (
                TransactionSubStatus::GasPriceTooLowForRbf,
                "GAS_PRICE_TOO_LOW_FOR_RBF",
            ),
            (
                TransactionSubStatus::IncompleteUserSetup,
                "INCOMPLETE_USER_SETUP",
            ),
            (
                TransactionSubStatus::InsufficientFunds,
                "INSUFFICIENT_FUNDS",
            ),
            (
                TransactionSubStatus::InsufficientFundsForFee,
                "INSUFFICIENT_FUNDS_FOR_FEE",
            ),
            (
                TransactionSubStatus::IntegrationSuspended,
                "INTEGRATION_SUSPENDED",
            ),
            (TransactionSubStatus::InvalidAddress, "INVALID_ADDRESS"),
            (
                TransactionSubStatus::InvalidContractCallData,
                "INVALID_CONTRACT_CALL_DATA",
            ),
            (TransactionSubStatus::InvalidFeeParams, "INVALID_FEE_PARAMS"),
            (
                TransactionSubStatus::InvalidNonceForRbf,
                "INVALID_NONCE_FOR_RBF",
            ),
            (
                TransactionSubStatus::InvalidTagOrMemo,
                "INVALID_TAG_OR_MEMO",
            ),
            (
                TransactionSubStatus::InvalidUnmanagedWallet,
                "INVALID_UNMANAGED_WALLET",
            ),
            (TransactionSubStatus::MaxFeeExceeded, "MAX_FEE_EXCEEDED"),
            (
                TransactionSubStatus::MissingTagOrMemo,
                "MISSING_TAG_OR_MEMO",
            ),
            (
                TransactionSubStatus::NeedMoreToCreateDestination,
                "NEED_MORE_TO_CREATE_DESTINATION",
            ),
            (
                TransactionSubStatus::NoMorePreprocessedIndexes,
                "NO_MORE_PREPROCESSED_INDEXES",
            ),
            (
                TransactionSubStatus::NonExistingAccountName,
                "NON_EXISTING_ACCOUNT_NAME",
            ),
            (
                TransactionSubStatus::RawMsgEmptyOrInvalid,
                "RAW_MSG_EMPTY_OR_INVALID",
            ),
            (
                TransactionSubStatus::RawMsgLenInvalid,
                "RAW_MSG_LEN_INVALID",
            ),
            (TransactionSubStatus::TooManyInputs, "TOO_MANY_INPUTS"),
            (
                TransactionSubStatus::TxSizeExceededMax,
                "TX_SIZE_EXCEEDED_MAX",
            ),
            (
                TransactionSubStatus::UnauthorisedDevice,
                "UNAUTHORISED_DEVICE",
            ),
            (TransactionSubStatus::UnauthorisedUser, "UNAUTHORISED_USER"),
            (
                TransactionSubStatus::UnallowedRawParamCombination,
                "UNALLOWED_RAW_PARAM_COMBINATION",
            ),
            (
                TransactionSubStatus::UnsupportedOperation,
                "UNSUPPORTED_OPERATION",
            ),
            (
                TransactionSubStatus::UnsupportedTransactionType,
                "UNSUPPORTED_TRANSACTION_TYPE",
            ),
            (
                TransactionSubStatus::ZeroBalanceInPermanentAddress,
                "ZERO_BALANCE_IN_PERMANENT_ADDRESS",
            ),
            (
                TransactionSubStatus::OutOfDateSigningKeys,
                "OUT_OF_DATE_SIGNING_KEYS",
            ),
            (
                TransactionSubStatus::ConnectivityError,
                "CONNECTIVITY_ERROR",
            ),
            (
                TransactionSubStatus::ErrorAsyncTxInFlight,
                "ERROR_ASYNC_TX_IN_FLIGHT",
            ),
            (TransactionSubStatus::InternalError, "INTERNAL_ERROR"),
            (
                TransactionSubStatus::InvalidNonceTooHigh,
                "INVALID_NONCE_TOO_HIGH",
            ),
            (
                TransactionSubStatus::InvalidNonceTooLow,
                "INVALID_NONCE_TOO_LOW",
            ),
            (
                TransactionSubStatus::InvalidRoutingDestination,
                "INVALID_ROUTING_DESTINATION",
            ),
            (
                TransactionSubStatus::LockingNonceAccountTimeout,
                "LOCKING_NONCE_ACCOUNT_TIMEOUT",
            ),
            (
                TransactionSubStatus::NetworkRoutingMismatch,
                "NETWORK_ROUTING_MISMATCH",
            ),
            (
                TransactionSubStatus::NonceAllocationFailed,
                "NONCE_ALLOCATION_FAILED",
            ),
            (
                TransactionSubStatus::ResourceAlreadyExists,
                "RESOURCE_ALREADY_EXISTS",
            ),
            (TransactionSubStatus::SignerNotFound, "SIGNER_NOT_FOUND"),
            (TransactionSubStatus::SigningError, "SIGNING_ERROR"),
            (TransactionSubStatus::Timeout, "TIMEOUT"),
            (TransactionSubStatus::TxOutdated, "TX_OUTDATED"),
            (TransactionSubStatus::UnknownError, "UNKNOWN_ERROR"),
            (
                TransactionSubStatus::VaultWalletNotReady,
                "VAULT_WALLET_NOT_READY",
            ),
            (
                TransactionSubStatus::UnsupportedMediaType,
                "UNSUPPORTED_MEDIA_TYPE",
            ),
            (
                TransactionSubStatus::AddressNotWhitelisted,
                "ADDRESS_NOT_WHITELISTED",
            ),
            (TransactionSubStatus::ApiKeyMismatch, "API_KEY_MISMATCH"),
            (
                TransactionSubStatus::AssetNotEnabledOnDestination,
                "ASSET_NOT_ENABLED_ON_DESTINATION",
            ),
            (
                TransactionSubStatus::DestTypeNotSupported,
                "DEST_TYPE_NOT_SUPPORTED",
            ),
            (
                TransactionSubStatus::ExceededDecimalPrecision,
                "EXCEEDED_DECIMAL_PRECISION",
            ),
            (
                TransactionSubStatus::ExchangeConfigurationMismatch,
                "EXCHANGE_CONFIGURATION_MISMATCH",
            ),
            (
                TransactionSubStatus::ExchangeVersionIncompatible,
                "EXCHANGE_VERSION_INCOMPATIBLE",
            ),
            (
                TransactionSubStatus::InvalidExchangeAccount,
                "INVALID_EXCHANGE_ACCOUNT",
            ),
            (TransactionSubStatus::MethodNotAllowed, "METHOD_NOT_ALLOWED"),
            (
                TransactionSubStatus::NonExistentAutoAccount,
                "NON_EXISTENT_AUTO_ACCOUNT",
            ),
            (
                TransactionSubStatus::OnPremiseConnectivityError,
                "ON_PREMISE_CONNECTIVITY_ERROR",
            ),
            (
                TransactionSubStatus::PeerAccountDoesNotExist,
                "PEER_ACCOUNT_DOES_NOT_EXIST",
            ),
            (
                TransactionSubStatus::ThirdPartyMissingAccount,
                "THIRD_PARTY_MISSING_ACCOUNT",
            ),
            (
                TransactionSubStatus::UnauthorisedIpWhitelisting,
                "UNAUTHORISED_IP_WHITELISTING",
            ),
            (
                TransactionSubStatus::UnauthorisedMissingCredentials,
                "UNAUTHORISED_MISSING_CREDENTIALS",
            ),
            (
                TransactionSubStatus::UnauthorisedMissingPermission,
                "UNAUTHORISED_MISSING_PERMISSION",
            ),
            (
                TransactionSubStatus::UnauthorisedOtpFailed,
                "UNAUTHORISED_OTP_FAILED",
            ),
            (TransactionSubStatus::WithdrawLimit, "WITHDRAW_LIMIT"),
            (
                TransactionSubStatus::Variant3RdPartyFailed,
                "3RD_PARTY_FAILED",
            ),
            (TransactionSubStatus::ApiCallLimit, "API_CALL_LIMIT"),
            (
                TransactionSubStatus::ApiInvalidSignature,
                "API_INVALID_SIGNATURE",
            ),
            (
                TransactionSubStatus::CancelledExternally,
                "CANCELLED_EXTERNALLY",
            ),
            (
                TransactionSubStatus::FailedAmlScreening,
                "FAILED_AML_SCREENING",
            ),
            (TransactionSubStatus::InvalidFee, "INVALID_FEE"),
            (
                TransactionSubStatus::InvalidThirdPartyResponse,
                "INVALID_THIRD_PARTY_RESPONSE",
            ),
            (
                TransactionSubStatus::ManualDepositAddressRequired,
                "MANUAL_DEPOSIT_ADDRESS_REQUIRED",
            ),
            (
                TransactionSubStatus::MissingDepositAddress,
                "MISSING_DEPOSIT_ADDRESS",
            ),
            (TransactionSubStatus::NoDepositAddress, "NO_DEPOSIT_ADDRESS"),
            (
                TransactionSubStatus::SubAccountsNotSupported,
                "SUB_ACCOUNTS_NOT_SUPPORTED",
            ),
            (
                TransactionSubStatus::SpendCoinbaseTooEarly,
                "SPEND_COINBASE_TOO_EARLY",
            ),
            (
                TransactionSubStatus::ThirdPartyInternalError,
                "THIRD_PARTY_INTERNAL_ERROR",
            ),
            (
                TransactionSubStatus::TxIdNotAcceptedByThirdParty,
                "TX_ID_NOT_ACCEPTED_BY_THIRD_PARTY",
            ),
            (TransactionSubStatus::UnsupportedAsset, "UNSUPPORTED_ASSET"),
            (TransactionSubStatus::DoubleSpending, "DOUBLE_SPENDING"),
            (
                TransactionSubStatus::DroppedByBlockchain,
                "DROPPED_BY_BLOCKCHAIN",
            ),
            (
                TransactionSubStatus::InsufficientReservedFunding,
                "INSUFFICIENT_RESERVED_FUNDING",
            ),
            (TransactionSubStatus::InvalidSignature, "INVALID_SIGNATURE"),
            (TransactionSubStatus::PartiallyFailed, "PARTIALLY_FAILED"),
            (
                TransactionSubStatus::PowerupSuggestionFailure,
                "POWERUP_SUGGESTION_FAILURE",
            ),
            (
                TransactionSubStatus::ReachedMempoolLimitForAccount,
                "REACHED_MEMPOOL_LIMIT_FOR_ACCOUNT",
            ),
            (
                TransactionSubStatus::RejectedByBlockchain,
                "REJECTED_BY_BLOCKCHAIN",
            ),
            (
                TransactionSubStatus::SmartContractExecutionFailed,
                "SMART_CONTRACT_EXECUTION_FAILED",
            ),
            (
                TransactionSubStatus::TooLongMempoolChain,
                "TOO_LONG_MEMPOOL_CHAIN",
            ),
            (TransactionSubStatus::Empty, ""),
        ];

        for (status, expected) in test_cases {
            assert_eq!(status.to_string(), expected);
        }
        assert_eq!(TransactionSubStatus::Empty, TransactionSubStatus::default());
    }
}
