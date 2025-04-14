use crate::datatypes::primitives::*;

pub struct AcceptableChannelCodings {
    pub value: String,
}
pub struct Action {
    pub value: String,
}

pub struct AddRoutingCategory {
    pub value: HDWord,
}

pub struct AgeOfEstimate {
    pub value: String,
}

pub struct AnswerTime {
    pub value: String,
}

pub struct AocIndicator {
    pub value: String,
}
pub struct ApplicationInfo {
    pub value: String,
}
pub struct BasicCallStateModel {
    pub value: String,
}

pub struct BasicServiceCode {
    pub value: String,
}

pub struct BasicServiceType {
    pub value: String,
}

pub struct BatchSeqNumber {
    pub value: String,
}

pub struct BearerServiceCode {
    pub value: String,
}

pub struct BlockSeqNumber {
    pub value: String,
}

pub struct BncConnectionType {
    pub value: String,
}

pub struct BIdleTime {
    pub value: String,
}

pub struct CallMedia {
    pub value: String,
}
pub struct CallReference {
    // word + word + byte
    pub value: String,
}

pub struct CallReferenceTime {
    pub value: String,
}
pub struct CallState {
    pub value: String,
}
pub struct CallType {
    pub value: String,
}

pub struct CallingNumber {
    pub value: String,
}

pub struct CallingPSTNCategory {
    pub value: String,
}

pub struct CamelCallReference {
    pub value: String,
}

pub struct CamelExchangeId {
    pub value: String,
}

pub struct CamelModifyParameters {
    pub value: String,
}

pub struct CamelModification {
    pub value: String,
}

pub struct CamelServiceKey {
    pub value: String,
}

pub struct CamelSMSModification {
    pub value: String,
}
pub struct CarrierSelection {
    pub value: String,
}
pub struct Category {
    pub value: String,
}
pub struct CauseForForwarding {
    pub value: String,
}
pub struct CauseForTermination {
    pub value: String,
}

pub struct CellBand {
    pub value: String,
}
pub struct CDBIndicator {
    pub value: String,
}
pub struct CfInformation {
    pub value: String,
}

pub struct ChangeDirection {
    pub value: String,
}
pub struct ChangePercent {
    pub value: String,
}

pub struct ChannelRateIndicator {
    pub value: String,
}

pub struct ChargeNature {
    pub value: String,
}

pub struct ChargingArea {
    pub value: String,
}
pub struct ChargingBlockSize {
    pub value: String,
}
pub struct CHARGING_END_TIME {
    //TODO
    pub value: String,
}

pub struct CHARGING_START_TIME {
    //TODO
    pub value: String,
}
pub struct CHARGING_TIME {
    //TODO
    pub value: String,
}

pub struct CHECK_SUM {
    //TODO
    pub value: String,
}

pub struct ChargeType {
    pub value: String,
}
pub struct CI {
    //TODO
    pub value: String,
}

pub struct CIP_CARRIER_CODE {
    //TODO
    pub value: String,
}

pub struct CLIENT_EXTERNAL_ID {
    //TODO
    pub value: String,
}
pub struct CommandType {
    pub value: String,
}

pub struct CONCATENATED_RECORD_NUMBER {
    //TODO
    pub value: String,
}

pub struct CONCATENATED_SMS_REFERENCE {
    //TODO
    pub value: String,
}

pub struct CONTROL_PLANE_INDEX {
    //TODO
    pub value: String,
}

pub struct CugInformation {
    pub value: String,
}
pub struct CugOutgoingAccess {
    pub value: String,
}

pub struct DATA_LENGTH_IN_BLOCK {
    //TODO
    pub value: String,
}

pub struct DATA_VOLUME {
    //TODO
    pub value: String,
}
pub struct DefaultCallHandling {
    pub value: String,
}
pub struct DefaultSmsHandling {
    pub value: String,
}

pub struct DELIVERY_TIME {
    //TODO
    pub value: String,
}

pub struct DeviceIdentifier {
    pub value: String,
}

pub struct DIALLED_DIGITS {
    //TODO
    pub value: String,
}
pub struct DisconnectingParty {
    pub value: String,
}

pub struct DtmfIndicator {
    pub value: String,
}

pub struct DURATION {
    //TODO
    pub value: String,
}
pub struct DURATION_BEFORE_ANSWER {
    //TODO
    pub value: String,
}
pub struct DURATION_BEFORE_ANSWER_TEN_MS {
    //TODO
    pub value: String,
}
pub struct DURATION_TEN_MS {
    //TODO
    pub value: String,
}
pub struct EllBand {
    pub value: String,
}

pub struct EMERGENCY_CALL_CATEGORY {
    //TODO
    pub value: String,
}

pub struct END_TIME {
    //TODO
    pub value: String,
}
pub struct EQUIPMENT_TYPE {
    //TODO
    pub value: String,
}

pub struct EQUIPMENT_ID {
    //TODO
    pub value: String,
}

pub struct EXCHANGE_ID {
    //TODO
    pub value: String,
}

pub struct EXIT_MSG_TIME {
    //TODO
    pub value: String,
}

pub struct EXIT_MSG_TRUNK_GROUP {
    //TODO
    pub value: String,
}

pub struct FACILITY_USAGE {
    //TODO
    pub value: String,
}

pub struct FIRST_RECORD_NUMBER {
    //TODO
    pub value: String,
}

pub struct FIXED_NW_USER_RATE {
    //TODO
    pub value: String,
}

pub struct FORMAT_VERSION {
    //TODO
    pub value: String,
}

pub struct FORWARDED_TO_SMSC {
    //TODO
    pub value: String,
}

pub struct GLOBAL_CALL_REFERENCE {
    //TODO
    pub value: String,
}
pub struct GMLC_ADDRESS {
    //TODO
    pub value: String,
}
pub struct GMSC_ADDRESS {
    //TODO
    pub value: String,
}
pub struct GPS_DATA {
    //TODO
    pub value: String,
}
pub struct GPS_DATA_LENGTH {
    //TODO
    pub value: String,
}
pub struct HORIZONTAL_ACCURACY {
    //TODO
    pub value: String,
}
pub struct HOT_BILLING_RECORD_NUMBER {
    //TODO
    pub value: String,
}
pub struct ICID_LENGTH {
    //TODO
    pub value: String,
}
pub struct ICID_OVERFLOW {
    //TODO
    pub value: String,
}
pub struct IMEI {
    //TODO
    pub value: String,
}
pub struct IMEISV {
    //TODO
    pub value: String,
}
pub struct IMSI {
    //TODO
    pub value: String,
}
pub struct IN_CATEGORY_KEY {
    //TODO
    pub value: String,
}
pub struct IN_CHANNEL_ALLOCATED_TIME {
    //TODO
    pub value: String,
}
pub struct IN_CIRCUIT_GROUP {
    //TODO
    pub value: String,
}
pub struct IN_CIRCUIT_GROUP_NAME {
    //TODO
    pub value: String,
}
pub struct IN_CIRCUIT {
    //TODO
    pub value: String,
}
pub struct IN_DATA {
    //TODO
    pub value: String,
}
pub struct IN_DATA_SPARE {
    //TODO
    pub value: String,
}
pub struct IN_DATA_LENGTH {
    //TODO
    pub value: String,
}
pub struct IN_DATA_LENGTH2 {
    //TODO
    pub value: String,
}
pub struct IN_LEG_ID {
    //TODO
    pub value: String,
}
pub struct IN_SERVICES {
    //TODO
    pub value: String,
}
pub struct INCOMING_TIME {
    //TODO
    pub value: String,
}
pub struct INITIATOR {
    //TODO
    pub value: String,
}
pub struct IN_RECORD_NUMBER {
    //TODO
    pub value: String,
}
pub struct IntermediateChargingInd {
    pub value: String,
}

pub struct INTERMEDIATE_CHRG_CAUSE {
    //TODO
    pub value: String,
}
pub struct INTERMEDIATE_RECORD_NUMBER {
    //TODO
    pub value: String,
}
pub struct JIP {
    //TODO
    pub value: String,
}
pub struct LAC {
    //TODO
    pub value: String,
}
pub struct LAST_EX_ID {
    //TODO
    pub value: String,
}
pub struct LAST_RECORD_NUMBER {
    //TODO
    pub value: String,
}
pub struct LEG_CALL_REFERENCE {
    //TODO
    pub value: String,
}
pub struct LEVEL_OF_CAMEL_SERVICE {
    //TODO
    pub value: String,
}
pub struct LOC_UP_INDICATOR {
    //TODO
    pub value: String,
}
pub struct LOCATION_REQUEST_TYPE {
    //TODO
    pub value: String,
}
pub struct MCC {
    //TODO
    pub value: String,
}
pub struct MESSAGE_REFERENCE {
    //TODO
    pub value: String,
}
pub struct MESSAGE_SIZE {
    //TODO
    pub value: String,
}
pub struct MNC {
    //TODO
    pub value: String,
}
pub struct MODIFY_DIRECTION {
    //TODO
    pub value: String,
}
pub struct MODIFY_PARAMETERS {
    //TODO
    pub value: String,
}
pub struct MODIFY_PERCENT {
    //TODO
    pub value: String,
}
pub struct MSC_TYPE {
    //TODO
    pub value: String,
}
pub struct MSRN {
    //TODO
    pub value: String,
}
pub struct MS_CLASSMARK3 {
    //TODO
    pub value: String,
}
pub struct NON_TRANSPARENCY_INDICATOR {
    //TODO
    pub value: String,
}
pub struct NPDB_QUERY_STATUS {
    //TODO
    pub value: String,
}
pub struct NPI {
    //TODO
    pub value: String,
}
pub struct NUM_OF_CONCATENATED_SMS {
    //TODO
    pub value: String,
}
pub struct NUMBER {
    //TODO
    pub value: String,
}
pub struct NUMBER_OF_FORWARDINGS {
    //TODO
    pub value: String,
}
pub struct NUMBER_OF_ALL_IN_RECORDS {
    //TODO
    pub value: String,
}
pub struct NUMBER_OF_SS_RECORDS {
    //TODO
    pub value: String,
}
pub struct NUMBER_OF_TRANSACTIONS {
    //TODO
    pub value: String,
}
pub struct OPTIMAL_ROUTING_INDICATOR {
    //TODO
    pub value: String,
}
pub struct ORIG_DIALLING_CLASS {
    //TODO
    pub value: String,
}
pub struct OTHER_MODEM_TYPE {
    //TODO
    pub value: String,
}
pub struct OUT_CHANNEL_ALLOCATED_TIME {
    //TODO
    pub value: String,
}
pub struct OUT_CIRCUIT_GROUP {
    //TODO
    pub value: String,
}
pub struct OUT_CIRCUIT_GROUP_NAME {
    //TODO
    pub value: String,
}
pub struct OUT_CIRCUIT {
    //TODO
    pub value: String,
}
pub struct PARAMETERS_LENGTH {
    //TODO
    pub value: String,
}
pub struct PARTY_TO_CHARGE {
    //TODO
    pub value: String,
}
pub struct PIC {
    //TODO
    pub value: String,
}
pub struct PNI {
    //TODO
    pub value: String,
}
pub struct PORTED_IN {
    //TODO
    pub value: String,
}

pub struct PROTOCOL_IDENTIFICATION {
    //TODO
    pub value: String,
}

pub struct PULSES {
    //TODO
    pub value: String,
}

pub struct RADIO_NETWORK_TYPE {
    //TODO
    pub value: String,
}

pub struct RATE_ADAPTION {
    //TODO
    pub value: String,
}

pub struct RECORD_LENGTH {
    //TODO
    pub value: String,
}

pub struct RECORD_NUMBER {
    //TODO
    pub value: String,
}

pub struct RecordStatus {
    pub value: String,
}

pub struct RecordType {
    pub value: String,
}

pub struct REDIRECTED_INDICATOR {
    //TODO
    pub value: String,
}

pub struct REGIONAL_SUBS_INDICATOR {
    //TODO
    pub value: String,
}

pub struct REGIONAL_SUBS_LOCATION_TYPE {
    //TODO
    pub value: String,
}

pub struct RELEASE_TIME {
    //TODO
    pub value: String,
}

pub struct REQ_AIR_INTERFACE_USER_RATE {
    //TODO
    pub value: String,
}
pub struct REQ_NUMBER_OF_CHANNELS {
    //TODO
    pub value: String,
}
pub struct REQ_USER_INITIATED_MOD_IND {
    //TODO
    pub value: String,
}
pub struct RESPONSE_TIME {
    //TODO
    pub value: String,
}
pub struct RESULT_INDICATOR {
    //TODO
    pub value: String,
}
pub struct ROUTING_CATEGORY {
    //TODO
    pub value: String,
}
pub struct ROUTING_INFO {
    //TODO
    pub value: String,
}

pub struct ROUTING_NUMBER {
    //TODO
    pub value: String,
}
pub struct SCF_ADDRESS {
    //TODO
    pub value: String,
}
pub struct SCP_CONNECTION {
    //TODO
    pub value: String,
}
pub struct SelectedCodec {
    pub value: String,
}
pub struct SERVICE_CODE {
    //TODO
    pub value: String,
}

pub struct SERVICE_IDENTIFIER {
    //TODO
    pub value: String,
}
pub struct SERVICE_TIME {
    //TODO
    pub value: String,
}
pub struct SHORTENED_IN_SERVICES {
    //TODO
    pub value: String,
}
pub struct SIP_SIG_MODE {
    //TODO
    pub value: String,
}
pub struct SMS_CENTRE {
    //TODO
    pub value: String,
}

pub struct SMS_LENGTH {
    //TODO
    pub value: String,
}

pub struct SMS_TYPE {
    //TODO
    pub value: String,
}

pub struct SPEECH_VERSION {
    //TODO
    pub value: String,
}

pub struct START_TIME {
    //TODO
    pub value: String,
}

pub struct SS_RECORD_NUMBER {
    //TODO
    pub value: String,
}

pub struct STREAM_IDENTIFIER {
    //TODO
    pub value: String,
}
pub struct SUBS_OLD_EX_ID {
    //TODO
    pub value: String,
}
pub struct SELECTED_CODEC {
    //TODO
    pub value: String,
}
pub struct SUBS_NEW_EX_ID {
    //TODO
    pub value: String,
}
pub struct SUBS_ROAMING_STATUS {
    //TODO
    pub value: String,
}
pub struct SUPPLEMENTARY_SERVICE_CODE {
    //TODO
    pub value: String,
}
pub struct TAPE_BLOCK_TYPE {
    //TODO
    pub value: String,
}

pub struct TARIFF_CHANGE_CNT {
    //TODO
    pub value: String,
}

pub struct TARIFF_CLASS {
    //TODO
    pub value: String,
}

pub struct TeleserviceCode {
    pub value: String,
}

pub struct TNS_CARRIER_CODE {
    //TODO
    pub value: String,
}

pub struct TNS_CIRCUIT_CODE {
    //TODO
    pub value: String,
}

pub struct TON {
    //TODO
    pub value: String,
}

pub struct USED_AIR_INTERFACE_USER_RATE {
    //TODO
    pub value: String,
}

pub struct USED_CHANNEL_CODING {
    //TODO
    pub value: String,
}

pub struct USED_NUMBER_OF_CHANNELS {
    //TODO
    pub value: String,
}

pub struct USED_POSITION_METHOD {
    //TODO
    pub value: String,
}

pub struct USED_UTRAN_POS_METHOD {
    //TODO
    pub value: String,
}

pub struct USER_PLANE_INDEX {
    //TODO
    pub value: String,
}

pub struct VERTICAL_ACCURACY {
    //TODO
    pub value: String,
}

pub struct VIRTUAL_MSC_ID {
    //TODO
    pub value: String,
}
pub struct VMSC_NUMBER {
    //TODO
    pub value: String,
}
