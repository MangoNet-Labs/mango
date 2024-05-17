const concepts = [
	{
		type: 'category',
		label: 'Tokenomics',
		collapsed: true,
		items: [
			'concepts/tokenomics/equity-pool-gains',
			'concepts/tokenomics/gas-calculate',
			'concepts/tokenomics/staker-income',
			'concepts/tokenomics/validator-benefits',
		],
	},
	{
		type: 'category',
		label: 'Cryptography',
		collapsed: true,
		items: [
			'concepts/cryptography/hash',
		],
	},
	{
		type: 'category',
		label: 'Account',
		collapsed: true,
		items: [
			'concepts/account/address-space',
		],
	},
	{
		type: 'category',
		label: 'Mango Move',
		collapsed: true,
		items: [
			'concepts/move-language-features/overview',
			'concepts/move-language-features/move-basic-data-type',
			'concepts/move-language-features/move-string',
			'concepts/move-language-features/move-special-methods',
			'concepts/move-language-features/move-collections',
			'concepts/move-language-features/move-witness',
			'concepts/move-language-features/move-package-upgrades',
		],
	},
	{
		type: 'category',
		label: 'Object Model',
		collapsed: true,
		items: [
			'concepts/object-model/overview',
			'concepts/object-model/dynamic-fields',
			'concepts/object-model/ownership',
		],
	},
	{
		type: 'category',
		label: 'In Depth Tech',
		collapsed: true,
		items: [
			'concepts/in-depth-tech/consensus',
			'concepts/in-depth-tech/storage',
			'concepts/in-depth-tech/data-management',
			'concepts/in-depth-tech/transaction-ordering',
			'concepts/in-depth-tech/vm',
		],
	},
];

module.exports = concepts;
