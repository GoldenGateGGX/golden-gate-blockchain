use super::*;

const fn percent(x: i32) -> sp_arithmetic::FixedI64 {
	sp_arithmetic::FixedI64::from_rational(x as u128, 100)
}
use pallet_referenda::Curve;
const APP_ROOT: Curve = Curve::make_reciprocal(4, 28, percent(80), percent(50), percent(100));
const SUP_ROOT: Curve = Curve::make_linear(28, 28, percent(0), percent(50));
const APP_STAKING_ADMIN: Curve = Curve::make_linear(17, 28, percent(50), percent(100));
const SUP_STAKING_ADMIN: Curve =
	Curve::make_reciprocal(12, 28, percent(1), percent(0), percent(50));
const APP_TREASURER: Curve = Curve::make_reciprocal(4, 28, percent(80), percent(50), percent(100));
const SUP_TREASURER: Curve = Curve::make_linear(28, 28, percent(0), percent(50));
const APP_FELLOWSHIP_ADMIN: Curve = Curve::make_linear(17, 28, percent(50), percent(100));
const SUP_FELLOWSHIP_ADMIN: Curve =
	Curve::make_reciprocal(12, 28, percent(1), percent(0), percent(50));
const APP_GENERAL_ADMIN: Curve =
	Curve::make_reciprocal(4, 28, percent(80), percent(50), percent(100));
const SUP_GENERAL_ADMIN: Curve =
	Curve::make_reciprocal(7, 28, percent(10), percent(0), percent(50));
const APP_REFERENDUM_CANCELLER: Curve = Curve::make_linear(17, 28, percent(50), percent(100));
const SUP_REFERENDUM_CANCELLER: Curve =
	Curve::make_reciprocal(12, 28, percent(1), percent(0), percent(50));
const APP_REFERENDUM_KILLER: Curve = Curve::make_linear(17, 28, percent(50), percent(100));
const SUP_REFERENDUM_KILLER: Curve =
	Curve::make_reciprocal(12, 28, percent(1), percent(0), percent(50));
const APP_SMALL_TIPPER: Curve = Curve::make_linear(10, 28, percent(50), percent(100));
const SUP_SMALL_TIPPER: Curve = Curve::make_reciprocal(1, 28, percent(4), percent(0), percent(50));
const APP_BIG_TIPPER: Curve = Curve::make_linear(10, 28, percent(50), percent(100));
const SUP_BIG_TIPPER: Curve = Curve::make_reciprocal(8, 28, percent(1), percent(0), percent(50));
const APP_SMALL_SPENDER: Curve = Curve::make_linear(17, 28, percent(50), percent(100));
const SUP_SMALL_SPENDER: Curve =
	Curve::make_reciprocal(12, 28, percent(1), percent(0), percent(50));
const APP_MEDIUM_SPENDER: Curve = Curve::make_linear(23, 28, percent(50), percent(100));
const SUP_MEDIUM_SPENDER: Curve =
	Curve::make_reciprocal(16, 28, percent(1), percent(0), percent(50));
const APP_BIG_SPENDER: Curve = Curve::make_linear(28, 28, percent(50), percent(100));
const SUP_BIG_SPENDER: Curve = Curve::make_reciprocal(20, 28, percent(1), percent(0), percent(50));
const APP_WHITELISTED_CALLER: Curve =
	Curve::make_reciprocal(16, 28 * 24, percent(96), percent(50), percent(100));
const SUP_WHITELISTED_CALLER: Curve =
	Curve::make_reciprocal(1, 28, percent(20), percent(5), percent(50));

static TRACKS_DATA: once_cell::sync::Lazy<
	[(u16, pallet_referenda::TrackInfo<Balance, BlockNumber>); 13],
> = once_cell::sync::Lazy::new(|| {
	[
		(
			0,
			pallet_referenda::TrackInfo {
				name: "root",
				max_deciding: 1,
				decision_deposit: 100 * KGGX,
				prepare_period: 2 * Hours::get(),
				decision_period: 14 * Days::get(),
				confirm_period: 24 * Hours::get(),
				min_enactment_period: 24 * Hours::get(),
				min_approval: APP_ROOT,
				min_support: SUP_ROOT,
			},
		),
		(
			1,
			pallet_referenda::TrackInfo {
				name: "whitelisted_caller",
				max_deciding: 100,
				decision_deposit: 10 * KGGX,
				prepare_period: 30 * Minutes::get(),
				decision_period: 14 * Days::get(),
				confirm_period: 10 * Minutes::get(),
				min_enactment_period: 10 * Minutes::get(),
				min_approval: APP_WHITELISTED_CALLER,
				min_support: SUP_WHITELISTED_CALLER,
			},
		),
		(
			10,
			pallet_referenda::TrackInfo {
				name: "staking_admin",
				max_deciding: 10,
				decision_deposit: 5 * KGGX,
				prepare_period: 2 * Hours::get(),
				decision_period: 14 * Days::get(),
				confirm_period: 3 * Hours::get(),
				min_enactment_period: 10 * Minutes::get(),
				min_approval: APP_STAKING_ADMIN,
				min_support: SUP_STAKING_ADMIN,
			},
		),
		(
			11,
			pallet_referenda::TrackInfo {
				name: "treasurer",
				max_deciding: 10,
				decision_deposit: 1 * KGGX,
				prepare_period: 2 * Hours::get(),
				decision_period: 14 * Days::get(),
				confirm_period: 3 * Hours::get(),
				min_enactment_period: 24 * Hours::get(),
				min_approval: APP_TREASURER,
				min_support: SUP_TREASURER,
			},
		),
		(
			12,
			pallet_referenda::TrackInfo {
				name: "fellowship_admin",
				max_deciding: 10,
				decision_deposit: 5 * KGGX,
				prepare_period: 2 * Hours::get(),
				decision_period: 14 * Days::get(),
				confirm_period: 3 * Hours::get(),
				min_enactment_period: 10 * Minutes::get(),
				min_approval: APP_FELLOWSHIP_ADMIN,
				min_support: SUP_FELLOWSHIP_ADMIN,
			},
		),
		(
			13,
			pallet_referenda::TrackInfo {
				name: "general_admin",
				max_deciding: 10,
				decision_deposit: 5 * KGGX,
				prepare_period: 2 * Hours::get(),
				decision_period: 14 * Days::get(),
				confirm_period: 3 * Hours::get(),
				min_enactment_period: 10 * Minutes::get(),
				min_approval: APP_GENERAL_ADMIN,
				min_support: SUP_GENERAL_ADMIN,
			},
		),
		(
			20,
			pallet_referenda::TrackInfo {
				name: "referendum_canceller",
				max_deciding: 1_000,
				decision_deposit: 10 * KGGX,
				prepare_period: 2 * Hours::get(),
				decision_period: 7 * Days::get(),
				confirm_period: 3 * Hours::get(),
				min_enactment_period: 10 * Minutes::get(),
				min_approval: APP_REFERENDUM_CANCELLER,
				min_support: SUP_REFERENDUM_CANCELLER,
			},
		),
		(
			21,
			pallet_referenda::TrackInfo {
				name: "referendum_killer",
				max_deciding: 1_000,
				decision_deposit: 50 * KGGX,
				prepare_period: 2 * Hours::get(),
				decision_period: 14 * Days::get(),
				confirm_period: 3 * Hours::get(),
				min_enactment_period: 10 * Minutes::get(),
				min_approval: APP_REFERENDUM_KILLER,
				min_support: SUP_REFERENDUM_KILLER,
			},
		),
		(
			30,
			pallet_referenda::TrackInfo {
				name: "small_tipper",
				max_deciding: 200,
				decision_deposit: 1 * GGX,
				prepare_period: 1 * Minutes::get(),
				decision_period: 7 * Days::get(),
				confirm_period: 10 * Minutes::get(),
				min_enactment_period: 1 * Minutes::get(),
				min_approval: APP_SMALL_TIPPER,
				min_support: SUP_SMALL_TIPPER,
			},
		),
		(
			31,
			pallet_referenda::TrackInfo {
				name: "big_tipper",
				max_deciding: 100,
				decision_deposit: 10 * GGX,
				prepare_period: 10 * Minutes::get(),
				decision_period: 7 * Days::get(),
				confirm_period: 1 * Hours::get(),
				min_enactment_period: 10 * Minutes::get(),
				min_approval: APP_BIG_TIPPER,
				min_support: SUP_BIG_TIPPER,
			},
		),
		(
			32,
			pallet_referenda::TrackInfo {
				name: "small_spender",
				max_deciding: 50,
				decision_deposit: 100 * GGX,
				prepare_period: 4 * Hours::get(),
				decision_period: 14 * Days::get(),
				confirm_period: 12 * Hours::get(),
				min_enactment_period: 24 * Hours::get(),
				min_approval: APP_SMALL_SPENDER,
				min_support: SUP_SMALL_SPENDER,
			},
		),
		(
			33,
			pallet_referenda::TrackInfo {
				name: "medium_spender",
				max_deciding: 50,
				decision_deposit: 200 * GGX,
				prepare_period: 4 * Hours::get(),
				decision_period: 14 * Days::get(),
				confirm_period: 24 * Hours::get(),
				min_enactment_period: 24 * Hours::get(),
				min_approval: APP_MEDIUM_SPENDER,
				min_support: SUP_MEDIUM_SPENDER,
			},
		),
		(
			34,
			pallet_referenda::TrackInfo {
				name: "big_spender",
				max_deciding: 50,
				decision_deposit: 400 * GGX,
				prepare_period: 4 * Hours::get(),
				decision_period: 14 * Days::get(),
				confirm_period: 48 * Hours::get(),
				min_enactment_period: 24 * Hours::get(),
				min_approval: APP_BIG_SPENDER,
				min_support: SUP_BIG_SPENDER,
			},
		),
	]
});

pub struct TracksInfo;
impl pallet_referenda::TracksInfo<Balance, BlockNumber> for TracksInfo {
	type Id = u16;
	type RuntimeOrigin = <RuntimeOrigin as frame_support::traits::OriginTrait>::PalletsOrigin;
	fn tracks() -> &'static [(Self::Id, pallet_referenda::TrackInfo<Balance, BlockNumber>)] {
		&*TRACKS_DATA
	}
	fn track_for(id: &Self::RuntimeOrigin) -> Result<Self::Id, ()> {
		if let Ok(system_origin) = frame_system::RawOrigin::try_from(id.clone()) {
			match system_origin {
				frame_system::RawOrigin::Root => Ok(0),
				_ => Err(()),
			}
		} else if let Ok(custom_origin) = origins::Origin::try_from(id.clone()) {
			match custom_origin {
				origins::Origin::WhitelistedCaller => Ok(1),
				// General admin
				origins::Origin::StakingAdmin => Ok(10),
				origins::Origin::Treasurer => Ok(11),
				origins::Origin::FellowshipAdmin => Ok(12),
				origins::Origin::GeneralAdmin => Ok(13),
				// Referendum admins
				origins::Origin::ReferendumCanceller => Ok(20),
				origins::Origin::ReferendumKiller => Ok(21),
				// Limited treasury spenders
				origins::Origin::SmallTipper => Ok(30),
				origins::Origin::BigTipper => Ok(31),
				origins::Origin::SmallSpender => Ok(32),
				origins::Origin::MediumSpender => Ok(33),
				origins::Origin::BigSpender => Ok(34),
				_ => Err(()),
			}
		} else {
			Err(())
		}
	}
}
pallet_referenda::impl_tracksinfo_get!(TracksInfo, Balance, BlockNumber);
