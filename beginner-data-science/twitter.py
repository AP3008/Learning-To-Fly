#from nt import access
#import secrets
import tweepy
from textblob import TextBlob
import os
from dotenv import load_dotenv

# Load variables from .env file
load_dotenv()

consumerKey = os.getenv('CONSUEMR_KEY')
secretKey = os.getenv('SECRET_KEY')
accessKey = os.getenv('ACCESS_TOKEN')
secAccKey = os.getenv('ACCESS_SECRET')

auth = tweepy.OAuthHandler(consumerKey, secretKey)
auth.set_access_token(accessKey, secAccKey)

api = tweepy.API(auth)

publicTweets = api.search_tweets("Trump")

for tweet in publicTweets:
    print(tweet)
    analysis = TextBlob(tweet.text)
    print(analysis.sentiment)

