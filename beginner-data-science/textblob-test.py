from textblob import TextBlob

wiki = TextBlob("John is angry he doesn't get any matches!")
print(wiki.tags)
