import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Arbitrator {
  'id' : bigint,
  'name' : string,
  'email' : string,
  'expertise' : string,
  'is_active' : boolean,
  'phone' : string,
  'arbitratoridentity_id' : string,
}
export interface ArbitratorIdentity { 'id' : bigint, 'principal' : string }
export interface Buyer {
  'id' : bigint,
  'name' : string,
  'email' : string,
  'purchased_land_titles' : BigUint64Array | bigint[],
  'phone' : string,
  'buyeridentity_id' : string,
}
export interface BuyerIdentity { 'id' : bigint, 'principal' : string }
export interface Dispute {
  'id' : bigint,
  'raised_by' : bigint,
  'created_at' : bigint,
  'comment' : string,
  'land_title_id' : bigint,
  'resolved_at' : [] | [bigint],
}
export type Error = { 'TransferConflict' : { 'msg' : string } } |
  { 'InvalidInput' : { 'msg' : string } } |
  { 'NotFound' : { 'msg' : string } } |
  { 'Unauthorized' : { 'msg' : string } } |
  { 'AlreadyExists' : { 'msg' : string } };
export interface GovernmentOfficial {
  'id' : bigint,
  'name' : string,
  'email' : string,
  'officialidentity_id' : string,
  'is_active' : boolean,
  'phone' : string,
  'department' : string,
}
export interface LandTitle {
  'id' : bigint,
  'landowner_id' : bigint,
  'size' : number,
  'description' : string,
  'is_available' : boolean,
  'price' : bigint,
  'registered_at' : bigint,
  'location' : string,
}
export interface LandTransfer {
  'id' : bigint,
  'landowner_id' : bigint,
  'agreed_price' : bigint,
  'approved_at' : [] | [bigint],
  'transfer_status' : string,
  'buyer_id' : bigint,
  'initiated_at' : bigint,
  'land_title_id' : bigint,
}
export interface Landowner {
  'id' : bigint,
  'name' : string,
  'email' : string,
  'is_verified' : boolean,
  'phone' : string,
  'land_titles' : BigUint64Array | bigint[],
  'landowneridentity_id' : string,
}
export type Result = { 'Ok' : Arbitrator } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : ArbitratorIdentity } |
  { 'Err' : Error };
export type Result_10 = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : Buyer } |
  { 'Err' : Error };
export type Result_3 = { 'Ok' : BuyerIdentity } |
  { 'Err' : Error };
export type Result_4 = { 'Ok' : Dispute } |
  { 'Err' : Error };
export type Result_5 = { 'Ok' : GovernmentOfficial } |
  { 'Err' : Error };
export type Result_6 = { 'Ok' : LandTitle } |
  { 'Err' : Error };
export type Result_7 = { 'Ok' : LandTransfer } |
  { 'Err' : Error };
export type Result_8 = { 'Ok' : Landowner } |
  { 'Err' : Error };
export type Result_9 = { 'Ok' : TransactionRecord } |
  { 'Err' : Error };
export interface TransactionRecord {
  'id' : bigint,
  'transaction_type' : string,
  'timestamp' : bigint,
  'details' : string,
  'land_title_id' : bigint,
}
export interface _SERVICE {
  'add_arbitrator' : ActorMethod<
    [string, string, string, string, string],
    Result
  >,
  'add_arbitrator_identity' : ActorMethod<[string], Result_1>,
  'add_buyer' : ActorMethod<[string, string, string, string], Result_2>,
  'add_buyer_identity' : ActorMethod<[string], Result_3>,
  'add_dispute' : ActorMethod<[bigint, bigint, string], Result_4>,
  'add_government_official' : ActorMethod<
    [string, string, string, string, string],
    Result_5
  >,
  'add_land_title' : ActorMethod<
    [bigint, string, number, string, bigint],
    Result_6
  >,
  'add_land_transfer' : ActorMethod<[bigint, bigint, bigint, bigint], Result_7>,
  'add_landowner' : ActorMethod<[string, string, string, string], Result_8>,
  'add_landowner_identity' : ActorMethod<[string], Result_3>,
  'add_official_identity' : ActorMethod<[string], Result_3>,
  'add_transaction_record' : ActorMethod<[bigint, string, string], Result_9>,
  'delete_arbitrator' : ActorMethod<[bigint], Result_10>,
  'delete_arbitrator_identity' : ActorMethod<[bigint], Result_10>,
  'delete_buyer' : ActorMethod<[bigint], Result_10>,
  'delete_buyer_identity' : ActorMethod<[bigint], Result_10>,
  'delete_dispute' : ActorMethod<[bigint], Result_10>,
  'delete_government_official' : ActorMethod<[bigint], Result_10>,
  'delete_land_title' : ActorMethod<[bigint], Result_10>,
  'delete_land_transfer' : ActorMethod<[bigint], Result_10>,
  'delete_landowner' : ActorMethod<[bigint], Result_10>,
  'delete_landowner_identity' : ActorMethod<[bigint], Result_10>,
  'delete_official_identity' : ActorMethod<[bigint], Result_10>,
  'delete_transaction_record' : ActorMethod<[bigint], Result_10>,
  'get_arbitrator' : ActorMethod<[bigint], Result>,
  'get_arbitrator_identity' : ActorMethod<[bigint], Result_3>,
  'get_buyer' : ActorMethod<[bigint], Result_2>,
  'get_buyer_identity' : ActorMethod<[bigint], Result_3>,
  'get_dispute' : ActorMethod<[bigint], Result_4>,
  'get_land_title' : ActorMethod<[bigint], Result_6>,
  'get_land_transfer' : ActorMethod<[bigint], Result_7>,
  'get_landowner' : ActorMethod<[bigint], Result_8>,
  'get_landowner_identity' : ActorMethod<[bigint], Result_3>,
  'get_official' : ActorMethod<[bigint], Result_5>,
  'get_official_identity' : ActorMethod<[bigint], Result_3>,
  'get_transaction_record' : ActorMethod<[bigint], Result_9>,
  'update_arbitrator' : ActorMethod<
    [
      bigint,
      [] | [string],
      [] | [string],
      [] | [string],
      [] | [string],
      [] | [boolean],
    ],
    Result
  >,
  'update_buyer' : ActorMethod<
    [bigint, [] | [string], [] | [string], [] | [string]],
    Result_2
  >,
  'update_dispute' : ActorMethod<
    [bigint, [] | [string], [] | [bigint]],
    Result_4
  >,
  'update_government_official' : ActorMethod<
    [
      bigint,
      [] | [string],
      [] | [string],
      [] | [string],
      [] | [string],
      [] | [boolean],
    ],
    Result_5
  >,
  'update_land_title' : ActorMethod<
    [
      bigint,
      [] | [string],
      [] | [number],
      [] | [string],
      [] | [bigint],
      [] | [boolean],
    ],
    Result_6
  >,
  'update_land_transfer' : ActorMethod<
    [
      bigint,
      [] | [bigint],
      [] | [bigint],
      [] | [bigint],
      [] | [string],
      [] | [bigint],
    ],
    Result_7
  >,
  'update_landowner' : ActorMethod<
    [bigint, [] | [string], [] | [string], [] | [string], [] | [boolean]],
    Result_8
  >,
  'update_transaction_record' : ActorMethod<
    [bigint, [] | [string], [] | [string]],
    Result_9
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
