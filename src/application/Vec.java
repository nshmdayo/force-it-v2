package application3;

public class Vec {
	public double x, y, z;

	public Vec(){
		x = 0;
		y = 0;
		z = 0;
	}

	public Vec(double x, double y, double z){
		this.x = x;
		this.y = y;
		this.z = z;
	}

	public void add(Vec target){
		x += target.x;
		y += target.y;
		z += target.z;
	}

	public void sub(Vec target, Vec current){
		x = target.x - current.x;
		y = target.y - current.y;
		z = target.z - current.z;
	}

	public void mult(double k){
		x *= k;
		y *= k;
		z *= k;
	}

	public double dot(Vec another){
		return x * another.x + y * another.y + z * another.z;
	}

	public double mag(){
		return Math.pow(x, 2) + Math.pow(y, 2) + Math.pow(z, 2);
	}

	public double dist(){
		return Math.sqrt(mag());
	}

	public void normalize(){
		double d = dist();
		if(d != 0){
			x /= d;
			y /= d;
			z /= d;
		}
	}

	public void copy(Vec another){
		x = another.x;
		y = another.y;
		z = another.z;
	}

	public void reset() {
		x = 0;
		y = 0;
		z = 0;
	}
}
