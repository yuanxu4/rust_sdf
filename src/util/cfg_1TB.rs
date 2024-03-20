
pub const CH_BITS:u16 = 4;
pub const EP_BITS:u16 = 2;
pub const PL_BITS:u16 = 2;
pub const LN_BITS:u16 = 2;
pub const PG_BITS:u16 = 8;
pub const BL_BITS:u16 = 1;



pub const NVME_QUART_PLANE 	:u16   	= PL_BITS << 0;
pub const NVME_SINGLE_PLANE	:u16	= 0 << 0;	
pub const NVME_DUAL_PLANE	:u16	= 1 << 0;
pub const NVME_AES_DISABLE 	:u16	= 0 << 6;
pub const NVME_AES_ENABLE 	:u16	= 1 << 6;
pub const NVME_AES_KEY0 	:u16	= 0 << 2;
pub const NVME_AES_KEY1 	:u16	= 1 << 2;
pub const NVME_AES_KEY2 	:u16	= 2 << 2;
pub const NVME_AES_KEY3 	:u16	= 3 << 2;
pub const NVME_AES_KEY4 	:u16	= 4 << 2;
pub const NVME_AES_KEY5 	:u16	= 5 << 2;
pub const NVME_AES_KEY6 	:u16	= 6 << 2;
pub const NVME_AES_KEY7 	:u16	= 7 << 2;
pub const NVME_AES_KEY8 	:u16	= 8 << 2;
pub const NVME_AES_KEY9 	:u16	= 9 << 2;
pub const NVME_AES_KEYA 	:u16	= 10 << 2;
pub const NVME_AES_KEYB 	:u16	= 11 << 2;
pub const NVME_AES_KEYC 	:u16	= 12 << 2;
pub const NVME_AES_KEYD 	:u16	= 13 << 2;
pub const NVME_AES_KEYE 	:u16	= 14 << 2;
pub const NVME_AES_KEYF 	:u16	= 15 << 2;


