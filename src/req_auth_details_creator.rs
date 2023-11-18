use crate::models::ReqPay;
use crate::models::reqAuthDetails::{ReqAuthDetails, Head, Txn, Payee, Amount, Payer, Identity, Info, Rating, Ac, Detail}; // Add all the necessary structs


pub fn create_req_auth_details(req_pay: &ReqPay) -> ReqAuthDetails {
    println!("Inside create_req_auth_details");
    ReqAuthDetails {
        head: Head {
            msg_id: req_pay.head.msgId.clone(),
            org_id: req_pay.head.orgId.clone(),
            ts: req_pay.head.ts.clone(),
            ver: req_pay.head.ver.clone(),
        },
        txn: Txn {
            cust_ref: req_pay.txn.custRef.clone(),
            id:req_pay.txn.id.clone(),
            note: req_pay.txn.note.clone(),
            ref_id: req_pay.txn.refId.clone(),
            ref_url: req_pay.txn.refUrl.clone(),
            ts: req_pay.txn.ts.clone(),    
            txn_type: req_pay.txn.tx_type.clone(),
        },
        payees: req_pay.payee.Payee.iter().map(|payee_data| {
            Payee {
                addr: payee_data.addr.clone(),
                code: payee_data.code.clone(),
                name: payee_data.name.clone(),
                seq_num: payee_data.seqNum.clone(),
                payee_type: payee_data.payee_type.clone(),
                amount: Amount {
                    curr: payee_data.amount.curr.clone(),
                    value: payee_data.amount.value.clone(),
                },
            }
        }).collect(),
        payer: Payer {
            addr: req_pay.payer.addr.clone(),
            code: req_pay.payer.code.clone(),
            name: req_pay.payer.name.clone(),
            seq_num: req_pay.payer.seqNum.clone(),
            payer_type: req_pay.payer.payer_type.clone(),
            info: Info {
                identity: Identity {
                    id: req_pay.payer.Info.Identity.id.clone(),
                    identity_type: req_pay.payer.Info.Identity.identity_type.clone(),
                    verified_name: req_pay.payer.Info.Identity.verifiedName.clone(),
                },
                rating: Rating {
                    verified_address: req_pay.payer.Info.Rating.VerifiedAddress.clone(),
                },
            },
            ac: req_pay.payer.Ac.iter().map(|ac_data| Ac {
                addr_type: ac_data.addr_type.clone(),
                detail: ac_data.Detail.iter().map(|detail| Detail {
                    name: detail.name.clone(),
                    value: detail.value.clone(),
                }).collect(),
            }).collect(),
            amount: Amount {
                curr: req_pay.payer.Amount.curr.clone(),
                value: req_pay.payer.Amount.value.clone(),
            },
        },
    }
}
