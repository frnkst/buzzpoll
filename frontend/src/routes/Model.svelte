<script context="module" lang="ts">

  export type Poll = {
		id?: number,
    link?: string,
    question: string,
		answers: Answer[];
}

export type Answer = {
	id: string,
	text: string
	votes?: Vote[]
}

export type Vote = {
	clientId: string
}

export type PollResponse = {
	pollId: string
}

export type VoteRequest = {
	id: number,
	answer: Answer
}
</script>
