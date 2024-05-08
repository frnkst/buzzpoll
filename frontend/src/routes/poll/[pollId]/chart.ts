export const option = {
	xAxis: {
		max: 'dataMax'
	},
	yAxis: {
		type: 'category',
		data: [],
		inverse: true,
		animationDuration: 200,
		animationDurationUpdate: 200,
	},
	series: [
		{
			realtimeSort: true,
			name: 'X',
			type: 'bar',
			data: [],
			label: {
				show: true,
				position: 'right',
				valueAnimation: true
			}
		}
	],
	animationDuration: 0,
	animationDurationUpdate: 200,
	animationEasing: 'linear',
	animationEasingUpdate: 'linear'
};
