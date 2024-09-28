import json

with open('reducesDict.json', 'r') as f:
    data = json.load(f)

cur_dict = data

Translations = []
Errors = []

with open('wordstocheck.txt', 'r', encoding='utf-8') as thisFile:
    for line in thisFile:
        stringo = str(line).rstrip()
        #print(stringo)
        try:
            R_dict = next(item for item in cur_dict if item["simplified"] == stringo)
            MyString = R_dict['english']
            Translations.append(MyString)
        except Exception:
            #print("Error with" + stringo)
            Translations.append("Error")
            Errors.append(stringo)

with open('englishwords.txt', mode='wt', encoding='utf-8') as myfile:
    for k in range(len(Translations)):
        myfile.write(Translations[k] + '\n')

with open('errorwords.txt', mode ='wt', encoding ='utf-8') as otherFile:
    for k in range(len(Errors)):
        otherFile.write(Errors[k] + '\n')