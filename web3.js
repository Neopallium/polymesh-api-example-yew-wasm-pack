import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';

export async function enableExtensions(app) {
	const extensions = await web3Enable(app);
	//console.log(extensions);
	return extensions;
}

export async function getAccounts() {
	const accounts = await web3Accounts();
	console.log(accounts);
	return accounts;
}

export async function signPayload(payload) {
	const extension = await web3FromAddress(payload.address);
	//console.log(extension);
	const signer = extension.signer;
	const { signature } = await signer.signPayload(payload);
	//console.log(signature);
	return signature;
}
