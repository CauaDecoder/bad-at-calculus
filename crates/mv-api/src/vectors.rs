//! Vectors and analytic geometry commands. Vectors travel as `[x, y, z]`.

use mv_application::use_cases::{AnalyzePlane, AnalyzeVectors, PlaneRequest, VectorRequest};
use mv_domain::Vector3;
use mv_infra::parser::PrattParser;
use serde::{Deserialize, Serialize};

use crate::ApiError;

type VectorInputDto = [String; 3];
type VectorDto = [f64; 3];

fn dto(vector: Vector3) -> VectorDto {
    vector.components()
}

// ── vectors.analyze ─────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct VectorRequestDto {
    u: VectorInputDto,
    v: VectorInputDto,
    w: Option<VectorInputDto>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VectorReportDto {
    u: VectorDto,
    v: VectorDto,
    w: Option<VectorDto>,
    sum: VectorDto,
    difference: VectorDto,
    dot: f64,
    cross: VectorDto,
    norm_u: f64,
    norm_v: f64,
    unit_u: Option<VectorDto>,
    unit_v: Option<VectorDto>,
    angle: Option<f64>,
    projection_u_on_v: Option<VectorDto>,
    projection_v_on_u: Option<VectorDto>,
    parallelogram_area: f64,
    is_orthogonal: bool,
    is_parallel: bool,
    triple_product: Option<f64>,
}

pub fn analyze_vectors(request: VectorRequestDto) -> Result<VectorReportDto, ApiError> {
    let report = AnalyzeVectors::new(PrattParser).execute(&VectorRequest {
        u: request.u,
        v: request.v,
        w: request.w,
    })?;
    Ok(VectorReportDto {
        u: dto(report.u),
        v: dto(report.v),
        w: report.w.map(dto),
        sum: dto(report.sum),
        difference: dto(report.difference),
        dot: report.dot,
        cross: dto(report.cross),
        norm_u: report.norm_u,
        norm_v: report.norm_v,
        unit_u: report.unit_u.map(dto),
        unit_v: report.unit_v.map(dto),
        angle: report.angle,
        projection_u_on_v: report.projection_u_on_v.map(dto),
        projection_v_on_u: report.projection_v_on_u.map(dto),
        parallelogram_area: report.parallelogram_area,
        is_orthogonal: report.is_orthogonal,
        is_parallel: report.is_parallel,
        triple_product: report.triple_product,
    })
}

// ── geometry.plane ──────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct PlaneRequestDto {
    a: VectorInputDto,
    b: VectorInputDto,
    c: VectorInputDto,
    p: VectorInputDto,
}

/// ax + by + cz + d = 0 with `normal = (a, b, c)` and `offset = d`.
#[derive(Serialize)]
pub struct PlaneDto {
    normal: VectorDto,
    offset: f64,
}

#[derive(Serialize)]
pub struct LineDto {
    point: VectorDto,
    direction: VectorDto,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaneReportDto {
    a: VectorDto,
    b: VectorDto,
    c: VectorDto,
    p: VectorDto,
    ab: VectorDto,
    ac: VectorDto,
    plane: Option<PlaneDto>,
    distance_to_plane: Option<f64>,
    foot_on_plane: Option<VectorDto>,
    p_is_on_plane: Option<bool>,
    line_ab: Option<LineDto>,
    distance_to_line: Option<f64>,
    foot_on_line: Option<VectorDto>,
}

pub fn analyze_plane(request: PlaneRequestDto) -> Result<PlaneReportDto, ApiError> {
    let report = AnalyzePlane::new(PrattParser).execute(&PlaneRequest {
        a: request.a,
        b: request.b,
        c: request.c,
        p: request.p,
    })?;
    Ok(PlaneReportDto {
        a: dto(report.a),
        b: dto(report.b),
        c: dto(report.c),
        p: dto(report.p),
        ab: dto(report.ab),
        ac: dto(report.ac),
        plane: report.plane.map(|plane| PlaneDto {
            normal: dto(plane.normal),
            offset: plane.offset,
        }),
        distance_to_plane: report.distance_to_plane,
        foot_on_plane: report.foot_on_plane.map(dto),
        p_is_on_plane: report.p_is_on_plane,
        line_ab: report.line_ab.map(|line| LineDto {
            point: dto(line.point),
            direction: dto(line.direction),
        }),
        distance_to_line: report.distance_to_line,
        foot_on_line: report.foot_on_line.map(dto),
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{commands, dispatch, ApiError};

    #[test]
    fn vector_components_accept_expressions() -> Result<(), ApiError> {
        let response = dispatch(
            commands::VECTORS_ANALYZE,
            json!({ "u": ["1", "2", "3"], "v": ["4", "5", "6"], "w": null }),
        )?;
        assert_eq!(response["dot"], json!(32.0));
        assert_eq!(response["cross"], json!([-3.0, 6.0, -3.0]));
        Ok(())
    }

    #[test]
    fn invalid_component_reports_its_field() {
        let error = dispatch(
            commands::VECTORS_ANALYZE,
            json!({ "u": ["1", "y", "3"], "v": ["4", "5", "6"] }),
        )
        .err();
        assert_eq!(error.and_then(|error| error.field), Some("u.y".to_owned()));
    }
}
