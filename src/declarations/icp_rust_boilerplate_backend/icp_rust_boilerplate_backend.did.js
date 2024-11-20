export const idlFactory = ({ IDL }) => {
  const Arbitrator = IDL.Record({
    'id' : IDL.Nat64,
    'name' : IDL.Text,
    'email' : IDL.Text,
    'expertise' : IDL.Text,
    'is_active' : IDL.Bool,
    'phone' : IDL.Text,
    'arbitratoridentity_id' : IDL.Text,
  });
  const Error = IDL.Variant({
    'TransferConflict' : IDL.Record({ 'msg' : IDL.Text }),
    'InvalidInput' : IDL.Record({ 'msg' : IDL.Text }),
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
    'Unauthorized' : IDL.Record({ 'msg' : IDL.Text }),
    'AlreadyExists' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : Arbitrator, 'Err' : Error });
  const ArbitratorIdentity = IDL.Record({
    'id' : IDL.Nat64,
    'principal' : IDL.Text,
  });
  const Result_1 = IDL.Variant({ 'Ok' : ArbitratorIdentity, 'Err' : Error });
  const Buyer = IDL.Record({
    'id' : IDL.Nat64,
    'name' : IDL.Text,
    'email' : IDL.Text,
    'purchased_land_titles' : IDL.Vec(IDL.Nat64),
    'phone' : IDL.Text,
    'buyeridentity_id' : IDL.Text,
  });
  const Result_2 = IDL.Variant({ 'Ok' : Buyer, 'Err' : Error });
  const BuyerIdentity = IDL.Record({
    'id' : IDL.Nat64,
    'principal' : IDL.Text,
  });
  const Result_3 = IDL.Variant({ 'Ok' : BuyerIdentity, 'Err' : Error });
  const Dispute = IDL.Record({
    'id' : IDL.Nat64,
    'raised_by' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'comment' : IDL.Text,
    'land_title_id' : IDL.Nat64,
    'resolved_at' : IDL.Opt(IDL.Nat64),
  });
  const Result_4 = IDL.Variant({ 'Ok' : Dispute, 'Err' : Error });
  const GovernmentOfficial = IDL.Record({
    'id' : IDL.Nat64,
    'name' : IDL.Text,
    'email' : IDL.Text,
    'officialidentity_id' : IDL.Text,
    'is_active' : IDL.Bool,
    'phone' : IDL.Text,
    'department' : IDL.Text,
  });
  const Result_5 = IDL.Variant({ 'Ok' : GovernmentOfficial, 'Err' : Error });
  const LandTitle = IDL.Record({
    'id' : IDL.Nat64,
    'landowner_id' : IDL.Nat64,
    'size' : IDL.Float64,
    'description' : IDL.Text,
    'is_available' : IDL.Bool,
    'price' : IDL.Nat64,
    'registered_at' : IDL.Nat64,
    'location' : IDL.Text,
  });
  const Result_6 = IDL.Variant({ 'Ok' : LandTitle, 'Err' : Error });
  const LandTransfer = IDL.Record({
    'id' : IDL.Nat64,
    'landowner_id' : IDL.Nat64,
    'agreed_price' : IDL.Nat64,
    'approved_at' : IDL.Opt(IDL.Nat64),
    'transfer_status' : IDL.Text,
    'buyer_id' : IDL.Nat64,
    'initiated_at' : IDL.Nat64,
    'land_title_id' : IDL.Nat64,
  });
  const Result_7 = IDL.Variant({ 'Ok' : LandTransfer, 'Err' : Error });
  const Landowner = IDL.Record({
    'id' : IDL.Nat64,
    'name' : IDL.Text,
    'email' : IDL.Text,
    'is_verified' : IDL.Bool,
    'phone' : IDL.Text,
    'land_titles' : IDL.Vec(IDL.Nat64),
    'landowneridentity_id' : IDL.Text,
  });
  const Result_8 = IDL.Variant({ 'Ok' : Landowner, 'Err' : Error });
  const TransactionRecord = IDL.Record({
    'id' : IDL.Nat64,
    'transaction_type' : IDL.Text,
    'timestamp' : IDL.Nat64,
    'details' : IDL.Text,
    'land_title_id' : IDL.Nat64,
  });
  const Result_9 = IDL.Variant({ 'Ok' : TransactionRecord, 'Err' : Error });
  const Result_10 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  return IDL.Service({
    'add_arbitrator' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text, IDL.Text, IDL.Text],
        [Result],
        [],
      ),
    'add_arbitrator_identity' : IDL.Func([IDL.Text], [Result_1], []),
    'add_buyer' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text, IDL.Text],
        [Result_2],
        [],
      ),
    'add_buyer_identity' : IDL.Func([IDL.Text], [Result_3], []),
    'add_dispute' : IDL.Func([IDL.Nat64, IDL.Nat64, IDL.Text], [Result_4], []),
    'add_government_official' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text, IDL.Text, IDL.Text],
        [Result_5],
        [],
      ),
    'add_land_title' : IDL.Func(
        [IDL.Nat64, IDL.Text, IDL.Float64, IDL.Text, IDL.Nat64],
        [Result_6],
        [],
      ),
    'add_land_transfer' : IDL.Func(
        [IDL.Nat64, IDL.Nat64, IDL.Nat64, IDL.Nat64],
        [Result_7],
        [],
      ),
    'add_landowner' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text, IDL.Text],
        [Result_8],
        [],
      ),
    'add_landowner_identity' : IDL.Func([IDL.Text], [Result_3], []),
    'add_official_identity' : IDL.Func([IDL.Text], [Result_3], []),
    'add_transaction_record' : IDL.Func(
        [IDL.Nat64, IDL.Text, IDL.Text],
        [Result_9],
        [],
      ),
    'delete_arbitrator' : IDL.Func([IDL.Nat64], [Result_10], []),
    'delete_arbitrator_identity' : IDL.Func([IDL.Nat64], [Result_10], []),
    'delete_buyer' : IDL.Func([IDL.Nat64], [Result_10], []),
    'delete_buyer_identity' : IDL.Func([IDL.Nat64], [Result_10], []),
    'delete_dispute' : IDL.Func([IDL.Nat64], [Result_10], []),
    'delete_government_official' : IDL.Func([IDL.Nat64], [Result_10], []),
    'delete_land_title' : IDL.Func([IDL.Nat64], [Result_10], []),
    'delete_land_transfer' : IDL.Func([IDL.Nat64], [Result_10], []),
    'delete_landowner' : IDL.Func([IDL.Nat64], [Result_10], []),
    'delete_landowner_identity' : IDL.Func([IDL.Nat64], [Result_10], []),
    'delete_official_identity' : IDL.Func([IDL.Nat64], [Result_10], []),
    'delete_transaction_record' : IDL.Func([IDL.Nat64], [Result_10], []),
    'get_arbitrator' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'get_arbitrator_identity' : IDL.Func([IDL.Nat64], [Result_3], ['query']),
    'get_buyer' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'get_buyer_identity' : IDL.Func([IDL.Nat64], [Result_3], ['query']),
    'get_dispute' : IDL.Func([IDL.Nat64], [Result_4], ['query']),
    'get_land_title' : IDL.Func([IDL.Nat64], [Result_6], ['query']),
    'get_land_transfer' : IDL.Func([IDL.Nat64], [Result_7], ['query']),
    'get_landowner' : IDL.Func([IDL.Nat64], [Result_8], ['query']),
    'get_landowner_identity' : IDL.Func([IDL.Nat64], [Result_3], ['query']),
    'get_official' : IDL.Func([IDL.Nat64], [Result_5], ['query']),
    'get_official_identity' : IDL.Func([IDL.Nat64], [Result_3], ['query']),
    'get_transaction_record' : IDL.Func([IDL.Nat64], [Result_9], ['query']),
    'update_arbitrator' : IDL.Func(
        [
          IDL.Nat64,
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Bool),
        ],
        [Result],
        [],
      ),
    'update_buyer' : IDL.Func(
        [IDL.Nat64, IDL.Opt(IDL.Text), IDL.Opt(IDL.Text), IDL.Opt(IDL.Text)],
        [Result_2],
        [],
      ),
    'update_dispute' : IDL.Func(
        [IDL.Nat64, IDL.Opt(IDL.Text), IDL.Opt(IDL.Nat64)],
        [Result_4],
        [],
      ),
    'update_government_official' : IDL.Func(
        [
          IDL.Nat64,
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Bool),
        ],
        [Result_5],
        [],
      ),
    'update_land_title' : IDL.Func(
        [
          IDL.Nat64,
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Float64),
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Nat64),
          IDL.Opt(IDL.Bool),
        ],
        [Result_6],
        [],
      ),
    'update_land_transfer' : IDL.Func(
        [
          IDL.Nat64,
          IDL.Opt(IDL.Nat64),
          IDL.Opt(IDL.Nat64),
          IDL.Opt(IDL.Nat64),
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Nat64),
        ],
        [Result_7],
        [],
      ),
    'update_landowner' : IDL.Func(
        [
          IDL.Nat64,
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Text),
          IDL.Opt(IDL.Bool),
        ],
        [Result_8],
        [],
      ),
    'update_transaction_record' : IDL.Func(
        [IDL.Nat64, IDL.Opt(IDL.Text), IDL.Opt(IDL.Text)],
        [Result_9],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
