from pathlib import Path
from pydub import AudioSegment
from operator import itemgetter

filename = Path("./list.txt")
train_file = Path('./list_train.txt')
val_file = Path('./list_val.txt')
path_wavs = Path('./wavs')
validation_share = 10

files = []

with open(filename, 'r', encoding='utf8') as list_input:
    for entries in list_input.readlines():
        file, text = entries.split('|')
        length = AudioSegment.from_file(file).duration_seconds
        files.append({'file': file, 'text': text, 'length': length})

amount_of_files = len(open(filename, 'r', encoding='utf8').readlines())
validation_data_amount = amount_of_files // validation_share
train_data_amount = amount_of_files - validation_data_amount

sorted_files = sorted(files, key=itemgetter('length'), reverse=True)
train_files = [f"{i['file']}|{i['text'].strip()}\n" for i  in sorted_files[validation_data_amount:]]
val_files = [f"{i['file']}|{i['text'].strip()}\n" for i  in sorted_files[:validation_data_amount]]

with open(train_file, 'w', encoding='utf8') as output_train, open(val_file, 'w', encoding='utf8') as output_val:
    output_train.writelines(train_files)
    output_val.writelines(val_files)
