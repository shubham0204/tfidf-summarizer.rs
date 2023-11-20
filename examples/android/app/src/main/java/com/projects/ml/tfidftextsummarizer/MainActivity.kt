package com.projects.ml.tfidftextsummarizer

import androidx.compose.material3.Checkbox
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ElevatedButton
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.projects.ml.summarizer.Summarizer
import com.projects.ml.tfidftextsummarizer.ui.theme.TFIDFTextSummarizerTheme

class MainActivity : ComponentActivity() {

    private val summarizer = Summarizer()
    private val documentText: MutableState<String> = mutableStateOf( "" )
    private var extractedSummary: MutableState<String> = mutableStateOf( "" )
    private var inferenceTime: Long = 0L

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            TFIDFTextSummarizerTheme {
                ActivityUI()
            }
        }
    }

    @Composable
    @Preview
    private fun ActivityUI() {
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = Color.White
        ) {
            Column(
                modifier = Modifier
                    .padding(horizontal = 24.dp)
                    .verticalScroll(rememberScrollState())
            ){
                val modifier = Modifier
                    .fillMaxWidth()
                Text(
                    modifier = modifier.padding( top = 16.dp ) ,
                    text = getString(R.string.app_name) ,
                    textAlign = TextAlign.Center ,
                    style = MaterialTheme.typography.titleMedium
                )
                Text(
                    modifier = modifier.padding( top = 4.dp , bottom = 24.dp ) ,
                    text = "Using TF-IDF scores to rank sentences" ,
                    textAlign = TextAlign.Center ,
                    style = MaterialTheme.typography.labelMedium
                )
                TextInput(modifier = modifier)
                Row(modifier = modifier
                    .padding(vertical = 16.dp)
                    .height(60.dp) ,
                    horizontalArrangement = Arrangement.Center ){
                    Buttons()
                }
                Summary(modifier = modifier)
            }
        }
    }

    @Composable
    private fun RowScope.Buttons() {
        val text by remember{ documentText }
        ElevatedButton(
            enabled = text.isNotEmpty(),
            modifier = Modifier
                .weight(1f)
                .fillMaxHeight()
                .padding(end = 4.dp),
            onClick = {
                extractedSummary.value = summarizer.compute( documentText.value )
            }
        ) { Text(text = "Summarize", textAlign = TextAlign.Center) }
        ElevatedButton(
            enabled = text.isNotEmpty(),
            modifier = Modifier
                .weight(1f)
                .fillMaxHeight()
                .padding(start = 4.dp),
            onClick = {
                val t1 = System.currentTimeMillis()
                extractedSummary.value = summarizer.compute( documentText.value )
                inferenceTime = System.currentTimeMillis() - t1
            }
        ) { Text(text = "Parallel Summarize", textAlign = TextAlign.Center) }
    }

    @OptIn(ExperimentalMaterial3Api::class)
    @Composable
    private fun TextInput( modifier: Modifier ) {
        var text by remember{ documentText }
        OutlinedTextField(
            modifier = modifier ,
            value = text,
            onValueChange = {
                text = it
            } ,
            maxLines = 12,
            textStyle = MaterialTheme.typography.bodyMedium ,
            label = { Text(text = "Text to be summarized") }
        )
    }

    @Composable
    private fun Summary( modifier: Modifier ) {
        val summary by remember{ extractedSummary }
        AnimatedVisibility(visible = summary.isNotEmpty()) {
            Card(
                modifier = modifier.padding( bottom = 24.dp ) ,
            ){
                val cardModifier = Modifier
                    .padding(horizontal = 16.dp)
                    .fillMaxWidth()
                Text(
                    modifier = cardModifier.padding( top = 16.dp , bottom = 2.dp ) ,
                    text = "Extracted Summary" ,
                    style = MaterialTheme.typography.titleMedium
                )
                Text(
                    modifier = cardModifier.padding( top = 16.dp , bottom = 8.dp ) ,
                    text = "Time taken $inferenceTime milliseconds" ,
                    style = MaterialTheme.typography.labelMedium
                )
                Text(
                    modifier = cardModifier.padding( bottom = 16.dp ) ,
                    text = summary ,
                    style = MaterialTheme.typography.bodyMedium
                )
            }
        }
    }

}

