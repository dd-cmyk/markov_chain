import os
import re
from nltk.tokenize import word_tokenize
story_path = "./4chan/"

def read_all_stories(story_path):
    txt = []
    for _, _, files in os.walk(story_path):
        for file in files:
            with open(story_path+file) as f:
                for line in f:
                    line = line.strip()
                    if line=='----------': break
                    if line!='':txt.append(line)
    return txt

def clean_txt(txt):
    cleaned_txt = []
    for line in txt:
        line = line.lower()
        line = re.sub(r"[,.\"\'!@#$%^&*(){}?/;`~:<>+=-\\]", "", line)
        tokens = word_tokenize(line)
        words = [word for word in tokens if word.isalpha()]
        cleaned_txt+=words
    return cleaned_txt

def save_to_file(txt):
    f = open("dataset.txt", "w")
    f.write(";".join(txt))
    f.close()

stories = read_all_stories(story_path)
print("number of lines = ", len(stories))
cleaned_stories = clean_txt(stories)
print("number of words = ", len(cleaned_stories))
save_to_file(cleaned_stories)
print("Saving to file dataset.txt")
