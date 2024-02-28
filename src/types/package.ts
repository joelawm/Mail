/**
 * Package is the structure of a build specification for the build system.
 * Mirrors the package_version.rs file.
 */
interface Package {
	target: string,
	version: string,
	description: string,
	rust_version: string,
	authors: string[],
	dependencies: {
		[key: string]: string | {
			version: string,
			features: string[],
		},
	},
}

export default Package;