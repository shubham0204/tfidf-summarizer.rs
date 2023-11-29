from summarizer import Summarizer

extractor = Summarizer()

with open( "wiki.txt" , "r" ) as file:
    text = file.read()
print( extractor.extract( text ) )