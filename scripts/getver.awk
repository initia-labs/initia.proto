{ 
	if ($1 == expr) { print $2; } 
	else if ($1 == "require" && $2 == expr) { print $3; }
}
