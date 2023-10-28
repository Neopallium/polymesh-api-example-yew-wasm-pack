import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';

export async function enableExtensions(app) {
	const extensions = await web3Enable(app);
	//console.log(extensions);
	return extensions;
}
