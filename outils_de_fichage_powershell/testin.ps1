$filesAut = Get-ChildItem 'C:\Users\sasha\OneDrive\Desktop\Testingpsh\Auteurs' #-Filter *.txt
$filesCours = Get-ChildItem 'C:\Users\sasha\OneDrive\Desktop\Testingpsh\Cours' #-Filter *.txt
foreach ($f in $filesAut){
	foreach ($_ in $filesCours){
		$thingy = '$f'.replace('.txt', '')
		(Get-Content $_.FullName).replace($thingy, '[$f]') | Set-Content $_.FullName
	}
}
