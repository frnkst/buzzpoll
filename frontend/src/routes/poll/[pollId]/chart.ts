export const option = {
	xAxis: {
		max: 'dataMax'
	},
	yAxis: {
		type: 'category',
		data: [],
		inverse: true,
		animationDuration: 300,
		animationDurationUpdate: 300,
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
	legend: {
		show: true
	},
	animationDuration: 0,
	animationDurationUpdate: 3000,
	animationEasing: 'linear',
	animationEasingUpdate: 'linear'
};
