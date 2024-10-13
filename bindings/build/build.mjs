import esbuild from 'esbuild';

async function main() {
    await esbuild.build({
        entryPoints: ['./src/index.mjs'],
        bundle: true,
        outfile: '../trading-charts/bindings/module.mjs',
        platform: 'node',
        minify: process.argv.includes('--minify'),
        format: 'esm',
        allowOverwrite: true,
    });
}

console.log('Building...');
main()
    .then(() => {console.log('Done.');})
    .catch((err) => {console.error(err);});
