const references = [
	{
		type: 'category',
		label: 'Mango CLI',
		collapsed: true,
		link: {
			type: 'doc',
			id: 'references/cli',
		},
		items: [
			'references/cli/client',
			'references/cli/console',
			'references/cli/keytool',
			'references/cli/move',
			'references/cli/validator',
		],
	},
	{
		type: 'category',
		label: 'Mango RPC',
		collapsed: true,
		link: {
			type: 'doc',
			id: 'references/mango-api',
		},
		items: [
			{
				type: 'link',
				label: 'JSON-RPC',
				href: '/mango-api-ref',
			},
		],
	},
	{
		type: 'category',
		label: 'Mango Contract',
		collapsed: true,
		items: [
			'references/contract/release-nft',
			'references/contract/release-token',
		],
	},
	{
		type: 'category',
		label: 'Mango Move',
		collapsed: true,
		items: [
			'references/move/move-lock',
			'references/move/move-toml',
		],
	},
	{
		type: 'category',
		label: 'Mango SDK',
		collapsed: true,
		items: [
			{
				type: 'category',
				label: 'TypeScript SDK',
				collapsed: true,
				items: [
					'references/sdk/ts-sdk/start',
					'references/sdk/ts-sdk/programmable-transaction-blocks',
					{
						type: 'category',
						label: 'Cryptography',
						collapsed: true,
						items: [
							'references/sdk/ts-sdk/cryptography/key-pairs',
							'references/sdk/ts-sdk/cryptography/multi-signature',
						],
					},
					'references/sdk/ts-sdk/utils',
				],
			},
			'references/sdk/wallet-standard',
		],
	},
];

module.exports = references;
