import fs from "fs";

import puppeteer from "puppeteer";

const words = fs
  .readFileSync("words.txt")
  .toString()
  .trim()
  .split("\n")
  .slice(1);

const browser = await puppeteer.launch({ headless: true });

let i = 0;
for (const wordsChunk of chunkArray(words, 50)) {
  if (i <= 16) {
    i += 1;
    continue;
  }
  const translations = await Promise.all(
    wordsChunk.map(async (word) => {
      const page = await browser.newPage();
      await page.goto(
        `https://translate.google.com/?sl=en&tl=zh-CN&text=${word}&op=translate`
      );
      await new Promise((resolve) => {
        page.on("response", (res) => {
          if (/log/gi.test(res._url)) setTimeout(resolve, 5000);
        });
      });
      const translations = { word };
      translations.main = await page.$eval(
        "[data-language-for-alternatives] span",
        (elt) => elt.innerHTML
      );
      translations.alts = await page.$$eval("[data-term-type]", (elts) =>
        elts.map((elt) => elt.innerHTML)
      );
      return translations;
    })
  );

  console.log("wrote chunk", i);
  fs.writeFileSync(`chunk_${i}.json`, JSON.stringify(translations));
  i += 1;
}

await browser.close();

function chunkArray(array, chunkSize) {
  return Array.from(
    { length: Math.ceil(array.length / chunkSize) },
    (_, index) => array.slice(index * chunkSize, (index + 1) * chunkSize)
  );
}
