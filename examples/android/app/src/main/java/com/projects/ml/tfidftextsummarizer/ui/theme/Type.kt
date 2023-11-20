package com.projects.ml.tfidftextsummarizer.ui.theme

import androidx.compose.material3.Typography
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.Font
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.sp
import com.projects.ml.tfidftextsummarizer.R

val AppFontFamily = FontFamily(
    Font(
        resId = R.font.inter_regular ,
        weight = FontWeight.Normal ,
        style = FontStyle.Normal
    ) ,
    Font(
        resId = R.font.inter_light ,
        weight = FontWeight.Light ,
        style = FontStyle.Normal
    ) ,
    Font(
        resId = R.font.inter_bold ,
        weight = FontWeight.Bold ,
        style = FontStyle.Normal
    ) ,
    Font(
        resId = R.font.inter_black ,
        weight = FontWeight.Black ,
        style = FontStyle.Normal
    )
)

val Typography = Typography(
    titleMedium = TextStyle(
        fontFamily = AppFontFamily ,
        fontSize = 22.sp
    ) ,
    labelMedium = TextStyle(
        fontFamily = AppFontFamily ,
        fontSize = 14.sp
    ) ,
    bodyMedium = TextStyle(
        fontFamily = AppFontFamily ,
        fontSize = 16.sp
    )
)