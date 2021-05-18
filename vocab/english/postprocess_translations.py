import json

with open('vocab_zh.tsv', 'w') as f_out:
    print('word\tword_detail\tdefinition', file=f_out)
    with open('translations.jsonl') as f_translations:
        for line in f_translations:
            translation = json.loads(line)
            alts = [alt for alt in translation['alts'] if not alt.isascii() and alt != translation['main']]
            selected_alts = [translation['main']] + alts[:4]
            print(f'{translation["word"]}\t\t{"/".join(selected_alts)}', file=f_out)
            #  print(json.dumps(translation, ensure_ascii=False), file=f_out)
