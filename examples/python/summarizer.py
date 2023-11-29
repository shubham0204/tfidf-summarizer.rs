import ctypes
import os

class Summarizer:

    def __init__( self ):
        self.lib = ctypes.CDLL( os.path.abspath( "libsummarizer.a" ) )


    def extract( 
        cls , 
        text: str , 
        reduction_factor: float = 0.6 , 
        multicore: bool = False
    ) -> str:
        return cls.__par_compute( text , reduction_factor ) \
                if multicore \
                else cls.__compute( text , reduction_factor )

    def __compute(
        self , 
        text: str , 
        reduction_factor: float
    ) -> str:
        return self.lib.summarize( text , len( text ) , reduction_factor )

    def __par_compute(
        self , 
        text: str , 
        reduction_factor: float
    ) -> str:
        return self.lib.par_summarize( text , len( text ) , reduction_factor )


        

