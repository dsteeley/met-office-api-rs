# spotdata_three_hourly_api_api

All URIs are relative to *https://api-metoffice.apiconnect.ibmcloud.com/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
**getThreeHourlyForecastUsingGET_1**](spotdata_three_hourly_api_api.md#getThreeHourlyForecastUsingGET_1) | **GET** /forecasts/point/three-hourly | Find nearest forecast data that matches the request criteria


# **getThreeHourlyForecastUsingGET_1**
> models::SpotForecastFeatureCollection getThreeHourlyForecastUsingGET_1(ctx, ctx, latitude, longitude, optional)
Find nearest forecast data that matches the request criteria

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **latitude** | **f64**| The request latitude value in range -85/+85 expressed as a decimal fraction. | 
  **longitude** | **f64**| The request longitude value in range -180/+180 expressed as a decimal fraction. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **latitude** | **f64**| The request latitude value in range -85/+85 expressed as a decimal fraction. | 
 **longitude** | **f64**| The request longitude value in range -180/+180 expressed as a decimal fraction. | 
 **exclude_parameter_metadata** | **bool**| If true, exclude parameter metadata in the response. If false or undefined, the parameter metadata is returned. | 
 **include_location_name** | **bool**| If true, include location name in the response. If false or undefined, no location name is returned. | 

### Return type

[**models::SpotForecastFeatureCollection**](SpotForecastFeatureCollection.md)

### Authorization

[client-id](../README.md#client-id), [client-secret](../README.md#client-secret)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

