import esbuild from 'esbuild';

async function main() {
    await esbuild.build({
        minify: process.argv.includes('--minify'),
        entryPoints: ['./src/index.mjs'],
        outfile: './target/module.mjs',
        platform: 'node',
        format: 'esm',
        bundle: true,
        allowOverwrite: true
    });
}

console.log('Building...');
main()
    .then(() => {console.log('Done.');})
    .catch((err) => {console.error(err);});
