package com.buzzpoll.model

data class VoteRequest(
    val id: Int,
    val answer: Answer,
    val clientId: String?
)

