#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]

pub struct AcceptableChannelCodings {
    pub value: String,
}
pub struct Action {
    pub value: String,
}

pub struct AddRoutingCategory {
    pub value: String,
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
pub struct ChargingEndtime {
    pub value: String,
}

pub struct ChargingStartTime {
    pub value: String,
}
pub struct ChargingTime {
    pub value: String,
}

pub struct CheckSum {
    pub value: String,
}

pub struct ChargeType {
    pub value: String,
}
pub struct CI {
    pub value: String,
}

pub struct CIPCarrierCode {
    pub value: String,
}

pub struct ClientExternalId {
    pub value: String,
}
pub struct CommandType {
    pub value: String,
}

pub struct ConcatenatedRecordNumber {
    pub value: String,
}

pub struct ConcatenatedSMSReference {
    pub value: String,
}

pub struct ControlPlaneIndex {
    pub value: String,
}

pub struct CugInformation {
    pub value: String,
}
pub struct CugOutgoingAccess {
    pub value: String,
}

pub struct DataLengthInBlock {
    pub value: String,
}

pub struct DataVolume {
    pub value: String,
}
pub struct DefaultCallHandling {
    pub value: String,
}
pub struct DefaultSmsHandling {
    pub value: String,
}

pub struct DeliveryTime {
    pub value: String,
}

pub struct DeviceIdentifier {
    pub value: String,
}

pub struct DialledDigits {
    pub value: String,
}
pub struct DisconnectingParty {
    pub value: String,
}

pub struct DtmfIndicator {
    pub value: String,
}

pub struct Duration {
    pub value: String,
}
pub struct DurationBeforeAnswer {
    pub value: String,
}
pub struct DurationBeforeAnswerTenMs {
    pub value: String,
}
pub struct DurationTenMs {
    pub value: String,
}
pub struct EllBand {
    pub value: String,
}

pub struct EmergencyCallCategory {
    pub value: String,
}

pub struct EndTime {
    pub value: String,
}
pub struct EquipmentType {
    pub value: String,
}

pub struct EquipmentId {
    pub value: String,
}

pub struct ExchangeId {
    pub value: String,
}

pub struct ExitMSGTime {
    pub value: String,
}

pub struct ExitMSGTrunkGroup {
    pub value: String,
}

pub struct FacilityUsage {
    pub value: String,
}

pub struct FirstRecordNumber {
    pub value: String,
}

pub struct FixedNWUserRate {
    pub value: String,
}

pub struct FormatVersion {
    pub value: String,
}

pub struct ForwardedToSMSC {
    pub value: String,
}

pub struct GlobalCallReference {
    pub value: String,
}
pub struct GMLCAddress {
    pub value: String,
}
pub struct GMSCAddress {
    pub value: String,
}
pub struct GPSData {
    pub value: String,
}
pub struct GPSDataLength {
    pub value: String,
}
pub struct HorizontalAccuracy {
    pub value: String,
}
pub struct HotBilingRecordNumber {
    pub value: String,
}
pub struct ICID {
    pub value: String,
}
pub struct ICIDLength {
    pub value: String,
}
pub struct ICIDOverflow {
    pub value: String,
}
pub struct IMEI {
    pub value: String,
}
pub struct IMEISV {
    pub value: String,
}
pub struct IMSI {
    pub value: String,
}
pub struct InCategoryKey {
    pub value: String,
}
pub struct InChannelAllocatedTime {
    pub value: String,
}
pub struct InCircuitGroup {
    pub value: String,
}
pub struct InCircuitGroupName {
    pub value: String,
}
pub struct InCircuit {
    pub value: String,
}
pub struct InData {
    // This field varies in length based on In data length
    pub value: String,
}
pub struct InDataSpare {
    pub value: String,
}
pub struct InDataLength {
    pub value: String,
}
pub struct InDataLength2 {
    pub value: String,
}
pub struct InLegId {
    pub value: String,
}
pub struct InServices {
    pub value: String,
}
pub struct IncomingTime {
    pub value: String,
}
pub struct Initiator {
    pub value: String,
}
pub struct InRecordNumber {
    pub value: String,
}
pub struct IntermediateChargingInd {
    pub value: String,
}

pub struct IntermediateChrgCause {
    pub value: String,
}
pub struct IntermediateRecordNumber {
    pub value: String,
}
pub struct JIP {
    pub value: String,
}
pub struct LAC {
    pub value: String,
}
pub struct LastExId {
    // CAMEL_EXCHANGE_ID_TON
    // CALLING_SUBS_LAST_EX_ID_TON
    // CALLED_SUBS_LAST_EX_ID_TON
    // FORWARDING_LAST_EX_ID_TON
    // FORWARDED_TO_LAST_EX_ID_TON
    // VIRTUAL_MSC_ID_TON
    pub value: String,
}
pub struct LastRecordNumber {
    pub value: String,
}
pub struct LegCallReference {
    pub value: String,
}
pub struct LevelOfCamelService {
    pub value: String,
}
pub struct LocUpIndicator {
    pub value: String,
}
pub struct LocationRequestType {
    pub value: String,
}

pub struct LocationEstimate {
    //TODO
    pub value: String,
}

pub struct MCC {
    pub value: String,
}
pub struct MessageReference {
    pub value: String,
}
pub struct MessageSize {
    pub value: String,
}
pub struct MNC {
    pub value: String,
}
pub struct ModifyDirection {
    pub value: String,
}
pub struct ModifyParameters {
    pub value: String,
}
pub struct ModifyPercent {
    pub value: String,
}
pub struct MSCType {
    pub value: String,
}
pub struct MSRN {
    pub value: String,
}
pub struct MSClassMark3 {
    pub value: String,
}
pub struct MSClassMark {
    pub value: String,
}
pub struct NonTrasnparencyIndicator {
    pub value: String,
}
pub struct NPDBQueryStatus {
    pub value: String,
}
pub struct NPI {
    pub value: String,
}
pub struct NumOfConcatenatedSMS {
    pub value: String,
}
pub struct Number {
    pub value: String,
}
pub struct NumberOfForwardings {
    pub value: String,
}
pub struct NumberOfAllInRecords {
    pub value: String,
}
pub struct NumberOfInRecords {
    pub value: String,
}
pub struct NumberOfSSRecords {
    pub value: String,
}
pub struct NumberOfTransactions {
    pub value: String,
}
pub struct OLI {
    pub value: String,
}
pub struct OptimalRoutingIndicator {
    pub value: String,
}
pub struct OrigDiallingClass {
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

pub struct RadioNetworkType {
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
pub struct RoutingCategory {
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
pub struct SMSCentre {
    pub value: String,
}

pub struct SMSLenght {
    pub value: String,
}

pub struct SMSType {
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
pub struct SubId {
    // subs_new_ex_id
    // subs_old_ex_id
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

pub struct TariffClass {
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
    // CALLING_NUMBER_TON
    // CALLED_NUMBER_TON
    // DIALLED_DIGITS_TON
    // INITIAL_ORIG_CALLED_NUMBER_TON
    // ORIG_CALLING_NUMBER_TON
    // FORWARDING_NUMBER_TON
    // FORWARDING_MSRN_TON
    // FORWARDED_TO_NUMBER_TON
    // CONNECTED_TO_NUMBER_TON
    // SERVED_NUMBER_TON
    // CALLED_MSRN_TON
    // FORWARDED_TO_MSRN_TON
    // CALLED_MSRN_TON
    // FORWARDED_TO_MSRN_TON
    // CHARGE_NUMBER_TON
    // OUTPULSED_NUMBER_TON
    // REDIRECTING_NUMBER_TON
    // ORIG_REDIRECTING_NUMBER_TON
    // DESTINATION_NUMBER_TON
    // LOC_ROUTING_NUMBER_TON
    // SCF_ADDRESS_TON
    // CLIENT_EXTERNAL_ID_TON
 
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
pub struct VMSCNumber {
    pub value: String,
}
