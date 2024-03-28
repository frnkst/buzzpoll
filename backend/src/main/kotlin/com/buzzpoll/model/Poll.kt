package com.buzzpoll.model

data class Poll (
    val id: Int?,
    val question: String,
    val answers: ArrayList<Answer>
)

data class Answer (
    val id: Int,
    val text: String,
    val votes: ArrayList<Vote>
)

data class Vote (
    val client: String
)
