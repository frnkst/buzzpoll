package com.buzzpoll.model

data class Poll (
    val id: Int?,
    val question: String,
    val answers: Array<Answer>
) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as Poll

        if (question != other.question) return false
        if (!answers.contentEquals(other.answers)) return false

        return true
    }

    override fun hashCode(): Int {
        var result = question.hashCode()
        result = 31 * result + answers.contentHashCode()
        return result
    }
}

data class Answer (
    val id: Int,
    val text: String,
    val votes: Array<Votes>
) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as Answer

        if (id != other.id) return false
        if (text != other.text) return false
        if (!votes.contentEquals(other.votes)) return false

        return true
    }

    override fun hashCode(): Int {
        var result = id
        result = 31 * result + text.hashCode()
        result = 31 * result + votes.contentHashCode()
        return result
    }
}

data class Votes (
    val client: String
)
