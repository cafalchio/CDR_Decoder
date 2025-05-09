pub mod blocks;
// block types
pub mod forw; // Forwarded Call
pub mod header; // header
pub mod hlri; // HLR Interrogation
pub mod in1; //Intelligent network data 1
pub mod in2; //Intelligent network data 2
pub mod in3; //Intelligent network data 3
pub mod in4; //Intelligent network data 4
pub mod loca; //Location Update CDR
pub mod moc; //Mobile-originated Call
pub mod poc; //PSTN-originated Call
pub mod ptc; //PSTN-terminated Call
pub mod smmo; //Short Message service (point-to-point), Mobile-originated
pub mod sups; //Supplementary Service
pub mod trailer;
pub mod uca; //Unsuccessful Call Attempt // trailer

// to be implemented
pub mod mtc; //Mobile-terminated Cal

// pub mod roam;     //Call to a Roaming Subscriber
pub mod coc; //Camel-originated Call
pub mod ctc; //Camel-terminated Cal
pub mod doc; //Device-originated Call
             // pub mod lcs;      //Location Services
             // pub mod pbxo;     //PBX-originated Cal
             // pub mod pbxt;     //PBX-terminated Call
             // pub mod rcc;      //Remote Charging Control
pub mod smmf;
pub mod smmt; //Short Message service (point-to-point), Mobile-terminated AI //Short Message service (point-to-point), Mobile-terminated AI

// pub mod ussd;     //Unstructured Supplementary Service Data
