#include "summarizer.h"
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <sys/time.h>

int main( int argc , char** argv ) {

    char* filename = argv[ 1 ] ; 

    FILE* file_ptr = fopen( filename , "r" ) ;
    fseek( file_ptr , 0 , SEEK_END ) ; 
      
    long size = ftell( file_ptr ) ; 

    fseek( file_ptr , 0 , SEEK_SET ) ; 
    char* buffer = (char*) calloc( size , sizeof(char) );  
    fread( buffer , sizeof( char ) , size , file_ptr ) ;
    fclose( file_ptr ) ;

    long start, end;
    struct timeval timecheck;

    gettimeofday(&timecheck, NULL);
    start = (long)timecheck.tv_sec * 1000 + (long)timecheck.tv_usec / 1000;

    const char* summarized_text = (char*) summarize( buffer , size , 0.5f ) ;

    gettimeofday(&timecheck, NULL);
    end = (long)timecheck.tv_sec * 1000 + (long)timecheck.tv_usec / 1000;
    
    printf( "%s \n" , summarized_text ) ;
    printf("%ld milliseconds elapsed\n", (end - start));
    
    return 0 ;
}