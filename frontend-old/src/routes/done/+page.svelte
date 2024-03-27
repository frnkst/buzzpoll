<script lang="ts">
	import { qrCode } from '$lib/stores/qrCode';

	let storedQRCode;
	qrCode.subscribe(value => {
		console.log("frank", value);
		return (storedQRCode = value);
	});
</script>

done

<h1>QR Code Data:</h1>
<p>{storedQRCode}</p>
