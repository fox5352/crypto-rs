const { exec } = require("child_process");
const path = require("path");
const util = require("util");
const execPromise = util.promisify(exec);

(async () => {
	try {
		await execPromise("mkdir -p dist");

		const { stdout: buildOut } = await execPromise("cargo build --release");
		console.log(buildOut);

		const dirName = path.basename(__dirname);
		const binaryPath = `target/release/${dirName}`;
		const outputPath = "dist/crypto-rs";

		await execPromise(`mv ${binaryPath} ${outputPath}`);
		console.log(`Moved binary to ${outputPath}`);
	} catch (err) {
		console.error(`Error: ${err.message}`);
		if (err.stderr) {
			console.error(`stderr: ${err.stderr}`);
		}
	}
})();
