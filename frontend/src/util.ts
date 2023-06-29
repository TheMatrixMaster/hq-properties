const download = (url: string) => {
	const a = document.createElement('a');
	a.href = url;
	a.target = '_blank';
	const download = url.split('/').pop();
	if (download) {
		a.download = download;
		document.body.appendChild(a);
		a.click();
		document.body.removeChild(a);
	}
};

const resetForm = (form_id: string) => {
	const f = document.getElementById(form_id) as HTMLFormElement;
	if (f) f.reset();
};

export { download, resetForm };
