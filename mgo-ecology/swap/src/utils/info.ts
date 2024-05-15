import mgo_img from '@/assets/image/img18.png';
// import help_img from '@/assets/image/help.png';
export const coinList = [
  {
    address: '0x2::mgo::MGO',
    coin: {
      decimal: 9
    },
    symbol: 'MGO',
    icon: mgo_img,
    Introduction: 'MGO token',
    balance: '0',
    disabled: false,
    isMain: false
  },
  {
    address: '0x96143722c8155746dc25cfd5c4664776d27441bfede174511018d0e46f4fea66::usdt::USDT',
    coin: {
      decimal: 2
    },
    symbol: 'USDT',
    icon: 'https://image.devnet.mangonetwork.io/img/usdt.png',
    Introduction: 'USDT token',
    balance: '0',
    disabled: false,
    isMain: false
  },
  {
    address: '0xa60797870293970fb8dc39915f4c60e1fc32d5a1bf5cb85308c24428d947567f::ai::AI',
    coin: {
      decimal: 9
    },
    symbol: 'AI',
    icon: 'https://image.devnet.mangonetwork.io/img/ai.png',
    Introduction: 'AI token',
    balance: '0',
    disabled: false,
    isMain: false
  }
]
export const contractAddress = '0x5ff2c7fb02e5eb9ed9175f47b2f7b6ea07099e43c7ef5d85b51de4f5372f38ef'
export const fullNode = 'https://fullnode.devnet.mangonetwork.io'
export const factoryAddress = '0xf5167798bd70acd9792738481a690a91ab45fdcdb768ec612a89cdc5ea06c8bc'
export const global_pause_status = '0x29520451803a6c7faeead3a8fadb23850859cc95d82b36b8290c06a65c6b3d75'
export const lp_front = '0x5ff2c7fb02e5eb9ed9175f47b2f7b6ea07099e43c7ef5d85b51de4f5372f38ef::amm_swap::LPCoin'
