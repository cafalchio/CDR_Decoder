pub mod blocks;
// block types
pub mod header;         // header
pub mod trailer;        // trailer
pub mod forw;           // Forwarded Call
pub mod hlri;           // HLR Interrogation
pub mod in1;            //Intelligent network data 1
pub mod in2;            //Intelligent network data 2
pub mod in3;            //Intelligent network data 3
pub mod in4;            //Intelligent network data 4
pub mod loca;           //Location Update CDR
pub mod moc;            //Mobile-originated Call
pub mod ptc;            //PSTN-terminated Call
pub mod smmo;           //Short Message service (point-to-point), Mobile-originated
pub mod sups;           //Supplementary Service
pub mod uca;            //Unsuccessful Call Attempt

// to be implemented
// pub mod coc;      //Camel-originated Call
// pub mod ctc;      //Camel-terminated Cal
// pub mod doc;      //Device-originated Call
// pub mod lcs;      //Location Services
// pub mod mtc;      //Mobile-terminated Cal
// pub mod pbxo;     //PBX-originated Cal
// pub mod pbxt;     //PBX-terminated Call
// pub mod rcc;      //Remote Charging Control
// pub mod roam;     //Call to a Roaming Subscriber
// pub mod smmf;     //Short Message service, Mobile-originated with Forwarding
// pub mod smmt;     //Short Message service (point-to-point), Mobile-terminated
// pub mod ussd;     //Unstructured Supplementary Service Data