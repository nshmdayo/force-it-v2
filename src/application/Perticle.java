package application3;

import javafx.scene.shape.Sphere;

public class Perticle extends Sphere{
	private Vec position, velocity, diff;
	long t;
	boolean delateFlag = false;
	double a = 1/(4*Main.d*(Main.d+1));

	public Perticle(double x , double y, double z) {
		setTranslateX(x);
		setTranslateY(y);
		setTranslateZ(z);

		position = new Vec(x, y, z);

		velocity = new Vec();

		diff = new Vec();
		t = System.currentTimeMillis();
	}

	public void move() {
		setTranslateX(getTranslateX() + velocity.x);
		setTranslateY(getTranslateY() + velocity.y);
		setTranslateZ(getTranslateZ() + velocity.z);
		diff.reset();
	}

	public void gravity() {
		if(getTranslateZ() < 0) {
			setTranslateX(position.x);
			setTranslateY(position.y);
			setTranslateZ(position.z);
			velocity.reset();
		}
		else {
			Vec a = new Vec(getTranslateX(), getTranslateY(), getTranslateZ());
			Vec v = new Vec();
			v.sub(position, a);
			v.mult(0.0005);
			velocity.add(v);
		}
	}

	public void setAroundMojule(Vec v) {
		diff.add(v);
	}

	public void moduleGravity() {
		diff.mult(a);
		velocity.add(diff);
	}

	public Vec getPosition() {
		return position;
	}

	public void addVelocity(Vec v) {
		velocity.add(v);
	}

	public Vec getVelocity() {
		return velocity;
	}
}
