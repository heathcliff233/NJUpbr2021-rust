#ifndef SPHERE_HPP
#define SPHERE_HPP

//==============================================================================
// Originally written in 2016 by Peter Shirley <ptrshrl@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all copyright
// and related and neighboring rights to this software to the public domain
// worldwide. This software is distributed without any warranty.
//
// You should have received a copy (see file COPYING.txt) of the CC0 Public
// Domain Dedication along with this software. If not, see
// <http://creativecommons.org/publicdomain/zero/1.0/>.
//==============================================================================

#include "hittable.hpp"
#include "vec3.hpp"

void get_sphere_uv(const vec3 &p, double &u, double &v) {
  auto phi = atan2(p.z(), p.x());
  auto theta = asin(p.y());
  u = 1 - (phi + pi) / (2 * pi);
  v = (theta + pi / 2) / pi;
}

class sphere : public hittable {
public:
  sphere() {}
  sphere(point3 cen, double r, shared_ptr<material> m)
      : center(cen), radius(r), mat_ptr(m){};

  virtual bool hit(const ray &r, double t_min, double t_max,
                   hit_record &rec) const;

  virtual bool bounding_box(double t0, double t1,
                            aabb &output_box) const override;

public:
  point3 center;
  double radius;
  shared_ptr<material> mat_ptr;
};

bool sphere::hit(const ray &r, double t_min, double t_max,
                 hit_record &rec) const {
  vec3 oc = r.origin() - center;
  auto a = r.direction().length_squared();
  auto half_b = dot(oc, r.direction());
  auto c = oc.length_squared() - radius * radius;
  auto discriminant = half_b * half_b - a * c;

  if (discriminant > 0) {
    auto root = sqrt(discriminant);
    auto temp = (-half_b - root) / a;
    if (temp < t_max && temp > t_min) {
      rec.t = temp;
      rec.p = r.at(rec.t);
      rec.normal = (rec.p - center) / radius;
      vec3 outward_normal = (rec.p - center) / radius;
      rec.set_face_normal(r, outward_normal);
      get_sphere_uv((rec.p - center) / radius, rec.u, rec.v);
      rec.mat_ptr = mat_ptr;
      return true;
    }
    temp = (-half_b + root) / a;
    if (temp < t_max && temp > t_min) {
      rec.t = temp;
      rec.p = r.at(rec.t);
      rec.normal = (rec.p - center) / radius;
      vec3 outward_normal = (rec.p - center) / radius;
      rec.set_face_normal(r, outward_normal);
      get_sphere_uv((rec.p - center) / radius, rec.u, rec.v);
      rec.mat_ptr = mat_ptr;
      return true;
    }
  }
  return false;
};

bool sphere::bounding_box(double t0, double t1, aabb &output_box) const {
  output_box = aabb(center - vec3(radius, radius, radius),
                    center + vec3(radius, radius, radius));
  return true;
}

#endif
