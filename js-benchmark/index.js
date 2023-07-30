const { Bench } = require('tinybench');
const fs = require('fs');
const path = require('path');
const bench = new Bench({ time: 250 });
const { parse, stringify } = require('postcss');
const tokenizer = require('postcss/lib/tokenize');
const mapGenerator = require('postcss/lib/map-generator');

const tinyFileString =  fs.readFileSync(path.join(__dirname, "../assets/bootstrap-rebot.css")).toString();
const biggerFileString =  fs.readFileSync(path.join(__dirname, "../assets/bootstrap.css")).toString();
const hugeFileString =  fs.readFileSync(path.join(__dirname, "../assets/tailwind-dark.css")).toString();
async function benchFun() {
    let tinyFileAST = parse(tinyFileString);;
    let biggerFileAST = parse(biggerFileString);
    let hugeFileAST = parse(hugeFileString);
    bench
    .add('postcss tokenize tiny file', () => {
        let t = tokenizer({css:tinyFileString});
        while (t.nextToken());
    })
    .add('postcss tokenize bigger file', () => {
        let t = tokenizer({css:biggerFileString});
        while (t.nextToken());
    })
    .add('postcss tokenize huge file', () => {
        let t = tokenizer({css:hugeFileString});
        while (t.nextToken());
    })
    .add('postcss parse tiny file', () => {
        parse(tinyFileString);
    })
    .add('postcss parse bigger file', () => {
        parse(biggerFileString);
    })
    .add('postcss parse huge file', () => {
        parse(hugeFileString);
    })
    .add('postcss generate tiny file', () => {
        let result = "";
        stringify(tinyFileAST, i => { result += i });
    })
    .add('postcss generate bigger file', () => {
        let result = "";
        stringify(biggerFileAST, i => { result += i });
    })
    .add('postcss generate huge file', () => {
        let result = "";
        stringify(hugeFileAST, i => { result += i });
    })
    .add('postcss generator tiny file with source map', async () => {
        let gen = new mapGenerator(stringify, tinyFileAST, { map: { inline: false } });
        gen.generate();
    })
    .add('postcss generator bigger file with source map', async () => {
        let gen = new mapGenerator(stringify, biggerFileAST, { map: { inline: false } });
        gen.generateMap();
    })
    .add('postcss generator huge file with source map', async () => {
        let gen = new mapGenerator(stringify, hugeFileAST, { map: { inline: false } });
        gen.generateMap();
    })

    await bench.run();
    console.table(bench.table());
}

benchFun();