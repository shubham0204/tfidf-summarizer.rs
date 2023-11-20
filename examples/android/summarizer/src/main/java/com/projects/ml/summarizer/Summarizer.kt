package com.projects.ml.summarizer

class Summarizer {

    companion object {
        init {
            System.loadLibrary( "summarizer" )
        }
    }

    fun extract(
        text: String ,
        reductionFactor: Float = 0.4f ,
        multicore: Boolean = false
    ) : String {
        return if( !multicore ) {
            summarize( text , reductionFactor )
        }
        else {
            parallelSummarize( text , reductionFactor )
        }
    }

    private external fun summarize( text: String , reductionFactor: Float ) : String
    private external fun parallelSummarize( text: String , reductionFactor: Float ) : String

}