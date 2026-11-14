import esbuild from 'esbuild';

const profile = process.env.PROFILE;
const node_env = process.env.NODE_ENV || '`undefined`';
const args = process.argv;
const minify = args.includes('--minify') || (args.includes('--auto') && profile !== 'debug');

async function main() {
    const options = {
        minify,
        entryPoints: ['./src/index.mjs'],
        outfile: './target/module.mjs',
        platform: 'node',
        format: 'esm',
        bundle: true,
        allowOverwrite: true,
    };

    if (!minify) {
        Object.assign(options, {
            keepNames: true,
            conditions: ['development', 'module'],
            define: {
                'process.env.DEBUG': JSON.stringify('true'),
                'process.env.NODE_ENV': JSON.stringify('development')
            }
        });
    }

    await esbuild.build(options);
}

console.log(`Building with profile env = '${profile}', node env ='${node_env}', and minify option = ${minify} ...`);
main()
    .then(() => {console.log('Done.');})
    .catch((err) => {console.error(err);});
